use std::collections::HashMap;

use actix::prelude::*;
use actix_broker::BrokerSubscribe;

use crate::message::{ChatMessage, JoinRoom, LeaveRoom, ListRooms, SendMessage};

type Client = Recipient<ChatMessage>;
type Room = HashMap<u64, Client>;

#[derive(Default)]
pub struct WsChatServer {
    rooms: HashMap<String, Room>,
}

impl WsChatServer {
    fn take_room(&mut self, room_name: &str) -> Option<Room> {
        let room = self.rooms.get_mut(room_name)?;
        let room = std::mem::take(room);
        Some(room)
    }

    fn add_client_to_room(&mut self, room_name: &str, id: Option<u64>, client: Client) -> u64 {
        let mut id = id.unwrap_or_else(rand::random);

        if let Some(room) = self.rooms.get_mut(room_name) {
            loop {
                if room.contains_key(&id) {
                    id = rand::random();
                } else {
                    break;
                }
            }

            room.insert(id, client);
            return id;
        }

        // Create a new room for the first client
        let mut room: Room = HashMap::new();

        room.insert(id, client);
        self.rooms.insert(room_name.to_owned(), room);

        id
    }

    fn send_chat_message(&mut self, room_name: &str, msg: &str, _src: u64) -> Option<()> {
        let mut room = self.take_room(room_name)?;

        for (id, client) in room.drain() {
            if client.try_send(ChatMessage(msg.to_owned())).is_ok() {
                self.add_client_to_room(room_name, Some(id), client);
            }
        }

        Some(())
    }
}

impl Actor for WsChatServer {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.subscribe_system_async::<LeaveRoom>(ctx);
        self.subscribe_system_async::<SendMessage>(ctx);
    }
}

impl Handler<JoinRoom> for WsChatServer {
    type Result = MessageResult<JoinRoom>;

    fn handle(&mut self, msg: JoinRoom, _ctx: &mut Self::Context) -> Self::Result {
        let JoinRoom(room_name, client_name, client) = msg;

        let id = self.add_client_to_room(&room_name, None, client);
        let join_msg = format!(
            "{} joined {room_name}",
            client_name.unwrap_or_else(|| "anon".to_owned()),
        );

        self.send_chat_message(&room_name, &join_msg, id);
        MessageResult(id)
    }
}

impl Handler<LeaveRoom> for WsChatServer {
    type Result = ();

    fn handle(&mut self, msg: LeaveRoom, _ctx: &mut Self::Context) {
        if let Some(room) = self.rooms.get_mut(&msg.0) {
            room.remove(&msg.1);
        }
    }
}

impl Handler<ListRooms> for WsChatServer {
    type Result = MessageResult<ListRooms>;

    fn handle(&mut self, _: ListRooms, _ctx: &mut Self::Context) -> Self::Result {
        MessageResult(self.rooms.keys().cloned().collect())
    }
}

impl Handler<SendMessage> for WsChatServer {
    type Result = ();

    fn handle(&mut self, msg: SendMessage, _ctx: &mut Self::Context) {
        let SendMessage(room_name, id, msg) = msg;
        self.send_chat_message(&room_name, &msg, id);
    }
}

impl SystemService for WsChatServer {}
impl Supervised for WsChatServer {}
