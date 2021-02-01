mod echo_service;
mod echo_rpc;
mod grpc_api;

use actix::prelude::*;
use log::{error, info};

use crate::echo_service::{EchoService, RunEcho};
use crate::echo_rpc::EchoRpc;

#[derive(Message)]
#[rtype(result = "()")]
pub struct SendEcho {
    pub payload: String,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct EchoReceived {
    pub payload: String,
}

struct EchoSender {
    service: Addr<EchoService>,
    echo_rpc: Option<Addr<EchoRpc>>,
}

impl EchoSender {
    fn new(service: Addr<EchoService>) -> EchoSender {
        EchoSender {
            service,
            echo_rpc: None,
        }
    }
}

impl Actor for EchoSender {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Context<Self>) {
        self.service.send(RunEcho { addr: ctx.address().recipient() })
            .into_actor(self)
            .map(|res, act, ctx| {
                match res {
                    Ok(Ok(echo_rpc)) =>  {
                        act.echo_rpc = Some(echo_rpc);
                    }
                    _ => {
                        error!("Unable to start echo RPC");
                        ctx.stop();
                    }
                }
            })
            .wait(ctx)
    }
}

impl Handler<SendEcho> for EchoSender {
    type Result = ();

    fn handle(&mut self, msg: SendEcho, ctx: &mut Context<Self>) {
        info!("Sending echo: {}", msg.payload);
        match &self.echo_rpc {
            Some(echo_rpc) => {
                echo_rpc.do_send(grpc_api::EchoRequest { payload: msg.payload });
            }
            None => {
                // Maybe we could do something smart like trying to (re)connect here.
                error!("Not connected!");
                ctx.stop();
            }
        }
    }
}

impl Handler<EchoReceived> for EchoSender {
    type Result = ();

    fn handle(&mut self, msg: EchoReceived, _: &mut Context<Self>) {
        info!("EchoSender has just received: {}", msg.payload)
    }
}

const ENDPOINT: &str = "http://127.0.0.1:50051";

#[actix_rt::main]
async fn main() {
    env_logger::init();

    let service = EchoService::new(ENDPOINT.to_string()).start();
    let sender = EchoSender::new(service).start();
    sender.do_send(SendEcho { payload: "Alpha".to_string() });
    sender.do_send(SendEcho { payload: "Beta".to_string() });
    sender.do_send(SendEcho { payload: "Gamma".to_string() });

    actix_rt::Arbiter::local_join().await;
}
