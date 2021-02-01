use actix::prelude::{
    Message as ActixMessage,
};

tonic::include_proto!("echo");

impl ActixMessage for EchoRequest {
    type Result = ();
}

impl ActixMessage for EchoReply {
    type Result = ();
}