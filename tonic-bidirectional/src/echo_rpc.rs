use actix::prelude::*;
use futures::channel::mpsc::Sender;
use log::info;

use crate::EchoReceived;
use crate::grpc_api::{
    EchoRequest,
    EchoReply,
};

pub struct EchoRpc {
    addr: actix::prelude::Recipient<EchoReceived>,
    tx: Sender<EchoRequest>,
}

impl Actor for EchoRpc {
    type Context = Context<Self>;
}

impl Handler<EchoRequest> for EchoRpc {
    type Result = ();

    fn handle(&mut self, msg: EchoRequest, ctx: &mut Context<Self>) {
        if let Err(_) = self.tx.try_send(msg) {
            info!("Sending echo request failed. Stopping.");
            ctx.stop();
        }
    }
}

impl StreamHandler<Result<EchoReply, tonic::Status>> for EchoRpc {
    fn handle(&mut self, msg: Result<EchoReply, tonic::Status>, _: &mut Context<Self>) {
        match msg {
            Ok(msg) => {
                self.addr.send(EchoReceived { payload: msg.payload });
            }
            Err(status) => {
                info!("Stream error: {}", status.message());
            }
        }
    }
}
