use actix::prelude::*;

#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct ChatMessage(pub String);

#[derive(Clone, Message)]
#[rtype(result = "u64")]
pub struct JoinRoom(pub String, pub Option<String>, pub Recipient<ChatMessage>);

#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct LeaveRoom(pub String, pub u64);

#[derive(Clone, Message)]
#[rtype(result = "Vec<String>")]
pub struct ListRooms;

#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct SendMessage(pub String, pub u64, pub String);
