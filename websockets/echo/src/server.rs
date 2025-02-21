use std::time::{Duration, Instant};

use actix_ws::AggregatedMessage;
use ractor::{ActorProcessingErr, ActorRef};

/// How often heartbeat pings are sent
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);

/// How long before lack of client response causes a timeout
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

#[derive(Debug)]
pub(crate) enum WsMessage {
    Ws(actix_ws::AggregatedMessage),
    Hb,
}

/// websocket connection is long running connection, it easier
/// to handle with an actor
pub(crate) struct MyWebSocket;

impl MyWebSocket {
    async fn handle_hb(
        &self,
        state: &mut (Instant, actix_ws::Session),
        myself: &ActorRef<WsMessage>,
    ) -> Result<(), ActorProcessingErr> {
        if Instant::now().duration_since(state.0) > CLIENT_TIMEOUT {
            // heartbeat timed out
            println!("Websocket Client heartbeat failed, disconnecting!");

            let _ = state.1.clone().close(None).await;
            myself.stop(None);

            // don't try to send a ping
        } else {
            state.1.ping(b"").await?;
        };

        Ok(())
    }

    async fn handle_ws_msg(
        &self,
        msg: AggregatedMessage,
        state: &mut (Instant, actix_ws::Session),
        myself: ActorRef<WsMessage>,
    ) -> Result<(), ActorProcessingErr> {
        println!("WS: {msg:?}");

        match msg {
            AggregatedMessage::Ping(msg) => {
                state.0 = Instant::now();
                state.1.pong(&msg).await?;
            }

            AggregatedMessage::Pong(_) => {
                state.0 = Instant::now();
            }

            AggregatedMessage::Text(text) => {
                state.1.text(text).await?;
            }

            AggregatedMessage::Binary(bin) => {
                state.1.binary(bin).await?;
            }

            AggregatedMessage::Close(reason) => {
                let _ = state.1.clone().close(reason).await;
                myself.stop(None);
            }
        };

        Ok(())
    }
}

impl ractor::Actor for MyWebSocket {
    type Msg = WsMessage;
    type State = (Instant, actix_ws::Session);
    type Arguments = actix_ws::Session;

    async fn pre_start(
        &self,
        myself: ActorRef<Self::Msg>,
        session: Self::Arguments,
    ) -> Result<Self::State, ActorProcessingErr> {
        myself.send_interval(HEARTBEAT_INTERVAL, || WsMessage::Hb);

        Ok((Instant::now(), session))
    }

    async fn handle(
        &self,
        myself: ActorRef<Self::Msg>,
        message: Self::Msg,
        state: &mut Self::State,
    ) -> Result<(), ActorProcessingErr> {
        match message {
            WsMessage::Hb => {
                self.handle_hb(state, &myself).await?;
            }

            WsMessage::Ws(msg) => {
                self.handle_ws_msg(msg, state, myself).await?;
            }
        }

        Ok(())
    }
}
