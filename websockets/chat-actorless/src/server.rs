//! A multi-room chat server.

use std::{
    collections::{HashMap, HashSet},
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
};

use rand::{thread_rng, Rng as _};
use tokio::sync::mpsc;

use crate::{Command, ConnId, Msg, RoomId};

/// A multi-room chat server.
#[derive(Debug)]
pub struct ChatServer {
    /// Map of connection IDs to their message receivers.
    sessions: HashMap<ConnId, mpsc::UnboundedSender<Msg>>,

    /// Map of room name to participant IDs in that room.
    rooms: HashMap<RoomId, HashSet<ConnId>>,

    /// Tracks total number of historical connections established.
    visitor_count: Arc<AtomicUsize>,

    /// Command receiver.
    rx: mpsc::UnboundedReceiver<Command>,
}

impl ChatServer {
    pub fn new() -> (Self, mpsc::UnboundedSender<Command>) {
        // create empty server
        let mut rooms = HashMap::with_capacity(4);

        // create default room
        rooms.insert("main".to_owned(), HashSet::new());

        let (tx, rx) = mpsc::unbounded_channel();

        (
            Self {
                sessions: HashMap::new(),
                rooms,
                visitor_count: Arc::new(AtomicUsize::new(0)),
                rx,
            },
            tx,
        )
    }
}

impl ChatServer {
    /// Send message to all users in the room.
    ///
    /// `skip_id` is used to prevent messages send by a connection also being received by it.
    async fn send_message(&self, room: &str, msg: impl Into<String>, skip_id: ConnId) {
        if let Some(sessions) = self.rooms.get(room) {
            let msg = msg.into();

            for conn_id in sessions {
                if *conn_id != skip_id {
                    if let Some(tx) = self.sessions.get(conn_id) {
                        tx.send(msg.clone()).unwrap();
                    }
                }
            }
        }
    }

    /// Handler for Connect message.
    ///
    /// Register new session and assign unique id to this session
    async fn connect(&mut self, tx: mpsc::UnboundedSender<Msg>) -> ConnId {
        log::info!("Someone joined");

        // notify all users in same room
        self.send_message("main", "Someone joined", 0).await;

        // register session with random connection ID
        let id = thread_rng().gen::<usize>();
        self.sessions.insert(id, tx);

        // auto join session to main room
        self.rooms
            .entry("main".to_owned())
            .or_insert_with(HashSet::new)
            .insert(id);

        let count = self.visitor_count.fetch_add(1, Ordering::SeqCst);
        self.send_message("main", format!("Total visitors {count}"), 0)
            .await;

        // send id back
        id
    }

    /// Handler for Disconnect message.
    async fn disconnect(&mut self, conn_id: ConnId) {
        println!("Someone disconnected");

        let mut rooms: Vec<String> = Vec::new();

        // remove sender
        if self.sessions.remove(&conn_id).is_some() {
            // remove session from all rooms
            for (name, sessions) in &mut self.rooms {
                if sessions.remove(&conn_id) {
                    rooms.push(name.to_owned());
                }
            }
        }

        // send message to other users
        for room in rooms {
            self.send_message(&room, "Someone disconnected", 0).await;
        }
    }

    /// Handler for `ListRooms` message.
    fn list_rooms(&mut self) -> Vec<String> {
        let mut rooms = Vec::new();

        for key in self.rooms.keys() {
            rooms.push(key.to_owned())
        }

        rooms
    }

    /// Join room, send disconnect message to old room send join message to new room.
    async fn join_room(&mut self, conn_id: ConnId, room: String) {
        let mut rooms = Vec::new();

        // remove session from all rooms
        for (n, sessions) in &mut self.rooms {
            if sessions.remove(&conn_id) {
                rooms.push(n.to_owned());
            }
        }
        // send message to other users
        for room in rooms {
            self.send_message(&room, "Someone disconnected", 0).await;
        }

        self.rooms
            .entry(room.clone())
            .or_insert_with(HashSet::new)
            .insert(conn_id);

        self.send_message(&room, "Someone connected", conn_id).await;
    }

    pub async fn run(mut self) {
        loop {
            match self.rx.recv().await.unwrap() {
                Command::Connect { conn_tx, res_tx } => {
                    let conn_id = self.connect(conn_tx).await;
                    res_tx.send(conn_id).unwrap();
                }

                Command::Disconnect { conn } => {
                    self.disconnect(conn).await;
                }

                Command::List { res_tx } => {
                    res_tx.send(self.list_rooms()).unwrap();
                }

                Command::Join { conn, room, res_tx } => {
                    self.join_room(conn, room).await;
                    res_tx.send(()).unwrap();
                }

                Command::Message {
                    room,
                    msg,
                    skip,
                    res_tx,
                } => {
                    self.send_message(&room, msg, skip).await;
                    res_tx.send(()).unwrap();
                }
            }
        }
    }
}
