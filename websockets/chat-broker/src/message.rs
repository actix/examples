use actix::prelude::*;

#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct ChatMessage(pub String);

#[derive(Clone, Message)]
#[rtype(result = "usize")]
pub struct JoinRoom(pub String, pub Option<String>, pub Recipient<ChatMessage>);

#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct LeaveRoom(pub String, pub usize);

#[derive(Clone, Message)]
#[rtype(result = "Vec<String>")]
pub struct ListRooms;

#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct SendMessage(pub String, pub usize, pub String);
