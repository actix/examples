use actix::prelude::*;
use log::{info, error};

use crate::EchoReceived;
use crate::echo_rpc::EchoRpc;
use crate::grpc_api::echo_client::EchoClient;

#[derive(Message)]
#[rtype(result = "Result<Addr<EchoRpc>, RunningEchoFailed>")]
pub struct RunEcho {
    pub addr: actix::prelude::Recipient<EchoReceived>,
}

#[derive(Debug)]
pub struct NotConnectedError;

#[derive(Debug)]
pub struct RunningEchoFailed;

pub struct EchoService {
    endpoint: String,
    client: Option<EchoClient<tonic::transport::Channel>>,
}

impl EchoService {
    pub fn new(endpoint: String) -> EchoService {
        EchoService {
            endpoint,
            client: None,
        }
    }
}

// EchoService should be responsible for connecting on startup and it'll have a RunEcho message

impl Actor for EchoService {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Context<Self>) {
        EchoClient::connect(self.endpoint.clone())
            .into_actor(self)
            .then(|res, act, ctx| {
                match res {
                    Ok(echo_client) => {
                        act.client = Some(echo_client);
                    }
                    Err(err) => {
                        error!("Unable to connect to echo server {:?}", err);
                        ctx.stop();
                    }
                }
                fut::ready(())
            })
            .wait(ctx)
    }
}

impl Handler<RunEcho> for EchoService {
    type Result = ResponseActFuture<Self, Result<Addr<EchoRpc>, RunningEchoFailed>>;

    fn handle(&mut self, msg: RunEcho, _ctx: &mut Context<Self>) -> Self::Result {
        if let Some(mut client) = &mut self.client {
            const OUTBOUND_CHANNEL_BUFFER: usize = 10;
            let (tx, rx) = futures::channel::mpsc::channel(OUTBOUND_CHANNEL_BUFFER);

            info!("Sending echo RPC!");
            Box::pin(
                client.echo(tonic::Request::new(rx))
                    .into_actor(self)
                    .map(|res, _act, _ctx| {
                        match res {
                            Ok(inbound) => {
                                Ok(EchoRpc::create(|ctx| {
                                    ctx.add_stream(inbound.into_inner());
                                    EchoRpc {
                                        addr: msg.addr,
                                        tx
                                    }
                                }))
                            }
                            Err(_) => {
                                // XXX: This is not really useful
                                Err(RunningEchoFailed)
                            }
                        }
                    })
            )
        } else {
            // XXX: do something smart about retrying. maybe ctx.stop()?
            error!("Not connected to the echo server");
            Box::pin(fut::err(RunningEchoFailed))
        }
    }
}

impl Supervised for EchoService {}
