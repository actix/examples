//! Ractor actor that manages TCP and WebSocket sessions and rooms.
//!
//! Routes messages to other sessions in the same room and replies to registration and room queries.

use std::collections::{HashMap, HashSet};

use ractor::{Actor, ActorProcessingErr, ActorRef, RpcReplyPort};
use rand::RngExt as _;
use tokio::sync::mpsc::UnboundedSender;

/// Messages accepted by the chat server actor.
pub enum Command {
    Connect {
        /// Channel for messages sent to this session.
        addr: UnboundedSender<String>,
        /// Reply port for the assigned session ID.
        reply: RpcReplyPort<u64>,
    },
    Disconnect {
        /// ID of the session to remove.
        id: u64,
    },
    ClientMessage {
        /// Sender's session ID, excluded from message delivery.
        id: u64,
        /// Text to send to the other sessions in the room.
        msg: String,
        /// Name of the destination room.
        room: String,
    },
    ListRooms(
        /// Reply port for the available room names.
        RpcReplyPort<Vec<String>>,
    ),
    Join {
        /// ID of the session to move.
        id: u64,
        /// Room to join, created if it does not exist.
        name: String,
    },
}

/// Shared room actor for TCP and WebSocket connections.
pub struct ChatServer;

/// Session channels and room membership owned by the chat actor.
#[derive(Debug)]
pub struct ChatState {
    sessions: HashMap<u64, UnboundedSender<String>>,
    rooms: HashMap<String, HashSet<u64>>,
}

impl ChatState {
    pub fn new() -> ChatState {
        // default room
        let mut rooms = HashMap::new();
        rooms.insert("main".to_owned(), HashSet::new());

        ChatState {
            sessions: HashMap::new(),
            rooms,
        }
    }
}

impl ChatState {
    /// Send message to all users in the room
    fn send_message(&self, room: &str, message: &str, skip_id: u64) {
        if let Some(sessions) = self.rooms.get(room) {
            for id in sessions {
                if *id != skip_id
                    && let Some(addr) = self.sessions.get(id)
                {
                    let _ = addr.send(message.to_owned());
                }
            }
        }
    }
}

impl Actor for ChatServer {
    type Msg = Command;
    type State = ChatState;
    type Arguments = ();

    async fn pre_start(
        &self,
        _: ActorRef<Command>,
        _: Self::Arguments,
    ) -> Result<ChatState, ActorProcessingErr> {
        Ok(ChatState::new())
    }

    async fn handle(
        &self,
        _: ActorRef<Command>,
        message: Command,
        state: &mut ChatState,
    ) -> Result<(), ActorProcessingErr> {
        match message {
            Command::Connect { addr, reply } => {
                tracing::info!("Someone joined");

                // notify all users in same room
                state.send_message("main", "Someone joined", 0);

                // register session with random id
                let id = rand::rng().random::<u64>();
                state.sessions.insert(id, addr);

                // auto join session to main room
                state.rooms.entry("main".to_owned()).or_default().insert(id);

                // send id back
                let _ = reply.send(id);
            }
            Command::Disconnect { id } => {
                tracing::info!("Someone disconnected");

                let mut rooms: Vec<String> = Vec::new();

                // remove address
                if state.sessions.remove(&id).is_some() {
                    // remove session from all rooms
                    for (name, sessions) in &mut state.rooms {
                        if sessions.remove(&id) {
                            rooms.push(name.to_owned());
                        }
                    }
                }
                // send message to other users
                for room in rooms {
                    state.send_message(&room, "Someone disconnected", 0);
                }
            }
            Command::ClientMessage { id, msg, room } => {
                state.send_message(&room, msg.as_str(), id);
            }
            Command::ListRooms(reply) => {
                let mut rooms = Vec::new();

                for key in state.rooms.keys() {
                    rooms.push(key.to_owned())
                }

                let _ = reply.send(rooms);
            }
            Command::Join { id, name } => {
                let mut rooms = Vec::new();

                // remove session from all rooms
                for (n, sessions) in &mut state.rooms {
                    if sessions.remove(&id) {
                        rooms.push(n.to_owned());
                    }
                }
                // send message to other users
                for room in rooms {
                    state.send_message(&room, "Someone disconnected", 0);
                }

                state.rooms.entry(name.clone()).or_default().insert(id);

                state.send_message(&name, "Someone connected", id);
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn rooms_route_messages_and_disconnect_sessions() {
        let (server, task) = ChatServer::spawn(None, ChatServer, ()).await.unwrap();
        let (a_tx, mut a_rx) = tokio::sync::mpsc::unbounded_channel();
        let a = ractor::call!(server, |reply| Command::Connect { addr: a_tx, reply }).unwrap();
        let (b_tx, mut b_rx) = tokio::sync::mpsc::unbounded_channel();
        let b = ractor::call!(server, |reply| Command::Connect { addr: b_tx, reply }).unwrap();

        while a_rx.try_recv().is_ok() {}
        while b_rx.try_recv().is_ok() {}

        server
            .cast(Command::ClientMessage {
                id: a,
                msg: "hello".into(),
                room: "main".into(),
            })
            .unwrap();
        ractor::call!(server, Command::ListRooms).unwrap();
        assert_eq!(b_rx.try_recv().unwrap(), "hello");
        assert!(a_rx.try_recv().is_err());

        server
            .cast(Command::Join {
                id: b,
                name: "other".into(),
            })
            .unwrap();
        let rooms = ractor::call!(server, Command::ListRooms).unwrap();
        assert!(rooms.contains(&"other".to_owned()));
        assert_eq!(a_rx.try_recv().unwrap(), "Someone disconnected");

        server
            .cast(Command::ClientMessage {
                id: a,
                msg: "private".into(),
                room: "main".into(),
            })
            .unwrap();
        ractor::call!(server, Command::ListRooms).unwrap();
        assert!(b_rx.try_recv().is_err());

        server.cast(Command::Disconnect { id: b }).unwrap();
        ractor::call!(server, Command::ListRooms).unwrap();
        assert!(b_rx.recv().await.is_none());

        server.stop(None);
        task.await.unwrap();
    }
}
