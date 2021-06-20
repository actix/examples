use std::collections::HashMap;

use actix::prelude::*;
use actix_broker::BrokerSubscribe;
use log::{debug, warn};

use crate::message::{
    ChatMessage, JoinRoom, LeaveRoom, ListClients, ListRooms, SendMessage,
};

type Client = Recipient<ChatMessage>;
type Room = HashMap<usize, Client>;

#[derive(Default)]
pub struct WsChatServer {
    rooms: HashMap<String, Room>,
}

impl WsChatServer {
    fn get_room(&mut self, room_name: &str) -> Option<&mut Room> {
        let room = self.rooms.get_mut(room_name)?;
        Some(room)
    }

    fn add_client_to_room(
        &mut self,
        room_name: &str,
        client_identifier: Option<usize>,
        client: Client,
    ) -> usize {
        let mut client_id = client_identifier.unwrap_or_else(rand::random::<usize>);

        if let Some(room) = self.rooms.get_mut(room_name) {
            debug!("add_client_to_room() - room found, {:?}", &room);

            loop {
                if room.contains_key(&client_id) {
                    warn!(
                        "add_client_to_room() - client id already here, creating new client id: {}",
                        &client_id
                    );
                    client_id = rand::random::<usize>();
                } else {
                    break;
                }
            }

            debug!(
                "add_client_to_room() - adding client to existing room, {}",
                &client_id
            );
            room.insert(client_id, client);
            return client_id;
        }

        // Create a new room for the first client
        let mut room: Room = HashMap::new();
        debug!(
            "add_client_to_room() - adding client to new room, {}",
            &client_id
        );

        room.insert(client_id, client);
        self.rooms.insert(room_name.to_owned(), room);

        client_id
    }

    fn send_chat_message(
        &mut self,
        room_name: &str,
        msg: &str,
        _src: usize,
    ) -> Option<()> {
        let room = self.get_room(room_name)?;

        for (_client_id, client) in room.iter() {
            client.do_send(ChatMessage(msg.to_owned())).ok()?;
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
        let name = client_name.unwrap_or_else(|| "anon".to_string());
        debug!(
            "JoinRoom::handle() - room_name: {}, client_name: {}",
            &room_name, &name
        );

        let id = self.add_client_to_room(&room_name, None, client);
        let join_msg = format!("{} joined {}", name, room_name);

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

impl Handler<ListClients> for WsChatServer {
    type Result = MessageResult<ListClients>;

    fn handle(&mut self, msg: ListClients, _ctx: &mut Self::Context) -> Self::Result {
        let ListClients(room_name) = msg;

        if let Some(room) = self.rooms.get(room_name.as_str()).cloned() {
            let client_names = room
                .keys()
                .map(|client_id| format!("{:?}", client_id))
                .collect();

            MessageResult(client_names)
        } else {
            panic!("ListClients::handle() - 404 room not found")
        }
    }
}

impl SystemService for WsChatServer {}
impl Supervised for WsChatServer {}
