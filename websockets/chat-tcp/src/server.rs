//! `ChatServer` is an actor. It maintains list of connection client session.
//! And manages available rooms. Peers send messages to other peers in same
//! room through `ChatServer`.

use std::collections::{HashMap, HashSet};

use actix::prelude::*;
use rand::Rng as _;

use crate::session;

/// Message for chat server communications
///
/// New chat session is created
#[derive(Message)]
#[rtype(u64)]
pub struct Connect {
    pub addr: Recipient<session::Message>,
}

/// Session is disconnected
#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: u64,
}

/// Send message to specific room
#[derive(Message)]
#[rtype(result = "()")]
pub struct Message {
    /// Id of the client session
    pub id: u64,
    /// Peer message
    pub msg: String,
    /// Room name
    pub room: String,
}

/// List of available rooms
pub struct ListRooms;

impl actix::Message for ListRooms {
    type Result = Vec<String>;
}

/// Join room, if room does not exists create new one.
#[derive(Message)]
#[rtype(result = "()")]
pub struct Join {
    /// Client id
    pub id: u64,
    /// Room name
    pub name: String,
}

/// `ChatServer` manages chat rooms and responsible for coordinating chat
/// session. implementation is super primitive
pub struct ChatServer {
    sessions: HashMap<u64, Recipient<session::Message>>,
    rooms: HashMap<String, HashSet<u64>>,
}

impl Default for ChatServer {
    fn default() -> ChatServer {
        // default room
        let mut rooms = HashMap::new();
        rooms.insert("main".to_owned(), HashSet::new());

        ChatServer {
            sessions: HashMap::new(),
            rooms,
        }
    }
}

impl ChatServer {
    /// Send message to all users in the room
    fn send_message(&self, room: &str, message: &str, skip_id: u64) {
        if let Some(sessions) = self.rooms.get(room) {
            for id in sessions {
                if *id != skip_id {
                    if let Some(addr) = self.sessions.get(id) {
                        addr.do_send(session::Message(message.to_owned()));
                    }
                }
            }
        }
    }
}

/// Make actor from `ChatServer`
impl Actor for ChatServer {
    /// We are going to use simple Context, we just need ability to communicate
    /// with other actors.
    type Context = Context<Self>;
}

/// Handler for Connect message.
///
/// Register new session and assign unique id to this session
impl Handler<Connect> for ChatServer {
    type Result = u64;

    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) -> Self::Result {
        println!("Someone joined");

        // notify all users in same room
        self.send_message("main", "Someone joined", 0);

        // register session with random id
        let id = rand::rng().random::<u64>();
        self.sessions.insert(id, msg.addr);

        // auto join session to main room
        self.rooms.get_mut("main").unwrap().insert(id);

        // send id back
        id
    }
}

/// Handler for Disconnect message.
impl Handler<Disconnect> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) {
        println!("Someone disconnected");

        let mut rooms: Vec<String> = Vec::new();

        // remove address
        if self.sessions.remove(&msg.id).is_some() {
            // remove session from all rooms
            for (name, sessions) in &mut self.rooms {
                if sessions.remove(&msg.id) {
                    rooms.push(name.to_owned());
                }
            }
        }
        // send message to other users
        for room in rooms {
            self.send_message(&room, "Someone disconnected", 0);
        }
    }
}

/// Handler for Message message.
impl Handler<Message> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: Message, _: &mut Context<Self>) {
        self.send_message(&msg.room, msg.msg.as_str(), msg.id);
    }
}

/// Handler for `ListRooms` message.
impl Handler<ListRooms> for ChatServer {
    type Result = MessageResult<ListRooms>;

    fn handle(&mut self, _: ListRooms, _: &mut Context<Self>) -> Self::Result {
        let mut rooms = Vec::new();

        for key in self.rooms.keys() {
            rooms.push(key.to_owned())
        }

        MessageResult(rooms)
    }
}

/// Join room, send disconnect message to old room
/// send join message to new room
impl Handler<Join> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: Join, _: &mut Context<Self>) {
        let Join { id, name } = msg;
        let mut rooms = Vec::new();

        // remove session from all rooms
        for (n, sessions) in &mut self.rooms {
            if sessions.remove(&id) {
                rooms.push(n.to_owned());
            }
        }
        // send message to other users
        for room in rooms {
            self.send_message(&room, "Someone disconnected", 0);
        }

        if self.rooms.get_mut(&name).is_none() {
            self.rooms.insert(name.clone(), HashSet::new());
        }
        self.send_message(&name, "Someone connected", id);
        self.rooms.get_mut(&name).unwrap().insert(id);
    }
}
