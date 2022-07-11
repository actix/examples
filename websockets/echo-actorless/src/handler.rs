use std::time::{Duration, Instant};

use actix_web::rt;
use actix_ws::Message;
use futures_util::stream::StreamExt as _;
use tokio::select;

/// How often heartbeat pings are sent
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);

/// How long before lack of client response causes a timeout
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

/// Echo text & binary messages received from the client, respond to ping messages, and monitor
/// connection health to detect network issues and free up resources.
pub async fn echo_heartbeat_ws(
    mut session: actix_ws::Session,
    mut msg_stream: actix_ws::MessageStream,
) {
    log::info!("connected");

    let mut last_heartbeat = Instant::now();

    let mut interval = rt::time::interval(HEARTBEAT_INTERVAL);

    loop {
        select! {
            Some(Ok(msg)) = msg_stream.next() => {
                log::debug!("msg: {msg:?}");

                match msg {
                    Message::Ping(bytes) => {
                        last_heartbeat = Instant::now();
                        let _ = session.pong(&bytes).await;
                    }

                    Message::Pong(_) => {
                        last_heartbeat = Instant::now();
                    }

                    Message::Text(text) => {
                        session.text(text).await.unwrap();
                    }

                    Message::Binary(bin) => {
                        session.binary(bin).await.unwrap();
                    }

                    Message::Close(reason) => {
                        let _ = session.close(reason).await;
                        break;
                    }

                    _ => {
                        let _ = session.close(None).await;
                        break;
                    }
                }
            }

            _ = interval.tick() => {
                // if no heartbeat ping/pong received recently, close the connection
                if Instant::now().duration_since(last_heartbeat) > CLIENT_TIMEOUT {
                    log::info!("client has not sent heartbeat in over {CLIENT_TIMEOUT:?}; disconnecting");
                    let _ = session.close(None).await;
                    break;
                }

                // send heartbeat ping
                let _ = session.ping(b"").await;

                // reset interval duration
                interval.reset();
            }
        };
    }
}

/// Echo text & binary messages received from the client and respond to ping messages.
///
/// This example is just for demonstration of simplicity. In reality, you likely want to include
/// some handling of heartbeats for connection health tracking to free up server resources when
/// connections die or network issues arise.
pub async fn echo_ws(mut session: actix_ws::Session, mut msg_stream: actix_ws::MessageStream) {
    log::info!("connected");

    while let Some(Ok(msg)) = msg_stream.next().await {
        log::debug!("msg: {msg:?}");

        match msg {
            Message::Ping(bytes) => {
                let _ = session.pong(&bytes).await;
            }

            Message::Pong(_) => {}

            Message::Text(text) => {
                session.text(text).await.unwrap();
            }

            Message::Binary(bin) => {
                session.binary(bin).await.unwrap();
            }

            Message::Close(reason) => {
                let _ = session.close(reason).await;
                break;
            }

            _ => {
                let _ = session.close(None).await;
                break;
            }
        }
    }
}
