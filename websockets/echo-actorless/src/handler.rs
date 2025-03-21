use std::time::{Duration, Instant};

use actix_web::web;
use actix_ws::Message;
use futures_util::{
    StreamExt as _,
    future::{self, Either},
};
use tokio::{pin, select, sync::broadcast, time::interval};

/// How often heartbeat pings are sent.
///
/// Should be half (or less) of the acceptable client timeout.
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);

/// How long before lack of client response causes a timeout.
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

/// Echo text & binary messages received from the client, respond to ping messages, and monitor
/// connection health to detect network issues and free up resources.
pub async fn echo_heartbeat_ws(
    mut session: actix_ws::Session,
    mut msg_stream: actix_ws::MessageStream,
) {
    log::info!("connected");

    let mut last_heartbeat = Instant::now();
    let mut interval = interval(HEARTBEAT_INTERVAL);

    let reason = loop {
        // create "next client timeout check" future
        let tick = interval.tick();
        // required for select()
        pin!(tick);

        // waits for either `msg_stream` to receive a message from the client or the heartbeat
        // interval timer to tick, yielding the value of whichever one is ready first
        match future::select(msg_stream.next(), tick).await {
            // received message from WebSocket client
            Either::Left((Some(Ok(msg)), _)) => {
                log::debug!("msg: {msg:?}");

                match msg {
                    Message::Text(text) => {
                        session.text(text).await.unwrap();
                    }

                    Message::Binary(bin) => {
                        session.binary(bin).await.unwrap();
                    }

                    Message::Close(reason) => {
                        break reason;
                    }

                    Message::Ping(bytes) => {
                        last_heartbeat = Instant::now();
                        let _ = session.pong(&bytes).await;
                    }

                    Message::Pong(_) => {
                        last_heartbeat = Instant::now();
                    }

                    Message::Continuation(_) => {
                        log::warn!("no support for continuation frames");
                    }

                    // no-op; ignore
                    Message::Nop => {}
                };
            }

            // client WebSocket stream error
            Either::Left((Some(Err(err)), _)) => {
                log::error!("{}", err);
                break None;
            }

            // client WebSocket stream ended
            Either::Left((None, _)) => break None,

            // heartbeat interval ticked
            Either::Right((_inst, _)) => {
                // if no heartbeat ping/pong received recently, close the connection
                if Instant::now().duration_since(last_heartbeat) > CLIENT_TIMEOUT {
                    log::info!(
                        "client has not sent heartbeat in over {CLIENT_TIMEOUT:?}; disconnecting"
                    );

                    break None;
                }

                // send heartbeat ping
                let _ = session.ping(b"").await;
            }
        }
    };

    // attempt to close connection gracefully
    let _ = session.close(reason).await;

    log::info!("disconnected");
}

/// Echo text & binary messages received from the client and respond to ping messages.
///
/// This example is just for demonstration of simplicity. In reality, you likely want to include
/// some handling of heartbeats for connection health tracking to free up server resources when
/// connections die or network issues arise.
///
/// See [`echo_heartbeat_ws`] for a more realistic implementation.
pub async fn echo_ws(mut session: actix_ws::Session, mut msg_stream: actix_ws::MessageStream) {
    log::info!("connected");

    let close_reason = loop {
        match msg_stream.next().await {
            Some(Ok(msg)) => {
                log::debug!("msg: {msg:?}");

                match msg {
                    Message::Text(text) => {
                        session.text(text).await.unwrap();
                    }

                    Message::Binary(bin) => {
                        session.binary(bin).await.unwrap();
                    }

                    Message::Close(reason) => {
                        break reason;
                    }

                    Message::Ping(bytes) => {
                        let _ = session.pong(&bytes).await;
                    }

                    Message::Pong(_) => {}

                    Message::Continuation(_) => {
                        log::warn!("no support for continuation frames");
                    }

                    // no-op; ignore
                    Message::Nop => {}
                };
            }

            // error or end of stream
            _ => break None,
        }
    };

    // attempt to close connection gracefully
    let _ = session.close(close_reason).await;

    log::info!("disconnected");
}

/// Broadcast text & binary messages received from a client, respond to ping messages, and monitor
/// connection health to detect network issues and free up resources.
pub async fn broadcast_ws(
    mut session: actix_ws::Session,
    mut msg_stream: actix_ws::MessageStream,
    mut rx: broadcast::Receiver<web::Bytes>,
) {
    log::info!("connected");

    let mut last_heartbeat = Instant::now();
    let mut interval = interval(HEARTBEAT_INTERVAL);

    let reason = loop {
        // waits for either `msg_stream` to receive a message from the client, the broadcast channel
        // to send a message, or the heartbeat interval timer to tick, yielding the value of
        // whichever one is ready first
        select! {
            broadcast_msg = rx.recv() => {
                let msg = match broadcast_msg {
                    Ok(msg) => msg,
                    Err(broadcast::error::RecvError::Closed) => break None,
                    Err(broadcast::error::RecvError::Lagged(_)) => continue,
                };

                let res = match std::str::from_utf8(&msg) {
                    Ok(val) => session.text(val).await,
                    Err(_) => session.binary(msg).await,
                };

                if let Err(err) = res {
                    log::error!("{err}");
                    break None;
                }
            }

            // heartbeat interval ticked
            _tick = interval.tick() => {
                // if no heartbeat ping/pong received recently, close the connection
                if Instant::now().duration_since(last_heartbeat) > CLIENT_TIMEOUT {
                    log::info!(
                        "client has not sent heartbeat in over {CLIENT_TIMEOUT:?}; disconnecting"
                    );

                    break None;
                }

                // send heartbeat ping
                let _ = session.ping(b"").await;
            },

            msg = msg_stream.next() => {
                let msg = match msg {
                    // received message from WebSocket client
                    Some(Ok(msg)) => msg,

                    // client WebSocket stream error
                    Some(Err(err)) => {
                        log::error!("{err}");
                        break None;
                    }

                    // client WebSocket stream ended
                    None => break None
                };

                log::debug!("msg: {msg:?}");

                match msg {
                    Message::Text(_) => {
                        // drop client's text messages
                    }

                    Message::Binary(_) => {
                        // drop client's binary messages
                    }

                    Message::Close(reason) => {
                        break reason;
                    }

                    Message::Ping(bytes) => {
                        last_heartbeat = Instant::now();
                        let _ = session.pong(&bytes).await;
                    }

                    Message::Pong(_) => {
                        last_heartbeat = Instant::now();
                    }

                    Message::Continuation(_) => {
                        log::warn!("no support for continuation frames");
                    }

                    // no-op; ignore
                    Message::Nop => {}
                };
            }
        }
    };

    // attempt to close connection gracefully
    let _ = session.close(reason).await;

    log::info!("disconnected");
}
