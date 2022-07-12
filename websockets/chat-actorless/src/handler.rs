use std::time::{Duration, Instant};

use actix_ws::Message;
use futures_util::{
    future::{select, Either},
    StreamExt as _,
};
use tokio::{
    pin,
    sync::{
        mpsc::{self, UnboundedSender},
        oneshot,
    },
    time::interval,
};

use crate::{Command, ConnId, RoomId};

/// How often heartbeat pings are sent
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);

/// How long before lack of client response causes a timeout
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

/// Echo text & binary messages received from the client, respond to ping messages, and monitor
/// connection health to detect network issues and free up resources.
pub async fn chat_ws(
    server_tx: UnboundedSender<Command>,
    mut session: actix_ws::Session,
    mut msg_stream: actix_ws::MessageStream,
) {
    log::info!("connected");

    let mut name = None;
    let mut room = "main".to_owned();
    let mut last_heartbeat = Instant::now();
    let mut interval = interval(HEARTBEAT_INTERVAL);

    let (conn_tx, mut conn_rx) = mpsc::unbounded_channel();
    let (res_tx, res_rx) = oneshot::channel();

    // unwrap: chat server is not dropped before the HTTP server
    server_tx
        .send(Command::Connect { conn_tx, res_tx })
        .unwrap();

    // unwrap: chat server does not drop our response channel
    let conn_id = res_rx.await.unwrap();

    let close_reason = loop {
        // most of the futures we process need to be stack-pinned to work with select()

        let tick = interval.tick();
        pin!(tick);

        let msg_rx = conn_rx.recv();
        pin!(msg_rx);

        let messages = select(msg_stream.next(), msg_rx);
        pin!(messages);

        match select(messages, tick).await {
            // commands & messages received from client
            Either::Left((Either::Left((Some(Ok(msg)), _)), _)) => {
                log::debug!("msg: {msg:?}");

                match msg {
                    Message::Ping(bytes) => {
                        last_heartbeat = Instant::now();
                        // unwrap:
                        session.pong(&bytes).await.unwrap();
                    }

                    Message::Pong(_) => {
                        last_heartbeat = Instant::now();
                    }

                    Message::Text(text) => {
                        process_text_msg(
                            &server_tx,
                            &mut session,
                            &text,
                            conn_id,
                            &mut room,
                            &mut name,
                        )
                        .await;
                    }

                    Message::Binary(_bin) => {
                        log::warn!("unexpected binary message");
                    }

                    Message::Close(reason) => break reason,

                    _ => {
                        break None;
                    }
                }
            }

            // client WebSocket stream error
            Either::Left((Either::Left((Some(Err(err)), _)), _)) => {
                log::error!("{}", err);
                break None;
            }

            // client WebSocket stream ended
            Either::Left((Either::Left((None, _)), _)) => break None,

            // chat messages received from other room participants
            Either::Left((Either::Right((Some(chat_msg), _)), _)) => {
                session.text(chat_msg).await.unwrap();
            }

            // all connection's msg senders were dropped
            Either::Left((Either::Right((None, _)), _)) => unreachable!(),

            // heartbeat internal tick
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
        };
    };

    // attempt to close connection gracefully
    let _ = session.close(close_reason).await;
}

async fn process_text_msg(
    server_tx: &UnboundedSender<Command>,
    session: &mut actix_ws::Session,
    text: &str,
    conn: ConnId,
    room: &mut RoomId,
    name: &mut Option<String>,
) {
    // strip leading and trailing whitespace (spaces, newlines, etc.)
    let msg = text.trim();

    // we check for /<cmd> type of messages
    if msg.starts_with('/') {
        let mut cmd_args = msg.splitn(2, ' ');

        // unwrap: we have guaranteed non-zero string length already
        match cmd_args.next().unwrap() {
            "/list" => {
                // Send ListRooms message to chat server and wait for
                // response
                log::info!("List rooms");

                let (res_tx, res_rx) = oneshot::channel();
                server_tx.send(Command::List { res_tx }).unwrap();
                // unwrap: chat server does not drop our response channel
                let rooms = res_rx.await.unwrap();

                for room in rooms {
                    session.text(room).await.unwrap();
                }
            }

            "/join" => match cmd_args.next() {
                Some(room_id) => {
                    *room = room_id.to_owned();

                    let (res_tx, res_rx) = oneshot::channel();

                    server_tx
                        .send(Command::Join {
                            conn,
                            room: room.clone(),
                            res_tx,
                        })
                        .unwrap();

                    // unwrap: chat server does not drop our response channel
                    res_rx.await.unwrap();

                    session.text(format!("joined {room_id}")).await.unwrap();
                }
                None => {
                    session.text("!!! room name is required").await.unwrap();
                }
            },

            "/name" => match cmd_args.next() {
                Some(new_name) => {
                    name.replace(new_name.to_owned());
                }
                None => {
                    session.text("!!! name is required").await.unwrap();
                }
            },

            _ => {
                session
                    .text(format!("!!! unknown command: {msg}"))
                    .await
                    .unwrap();
            }
        }
    } else {
        // prefix message with our name, if assigned
        let msg = match name {
            Some(ref name) => format!("{name}: {msg}"),
            None => msg.to_owned(),
        };

        let (res_tx, res_rx) = oneshot::channel();

        // send message to chat server
        server_tx
            .send(Command::Message {
                msg,
                room: room.clone(),
                skip: conn,
                res_tx,
            })
            // unwrap: chat server is not dropped before the HTTP server
            .unwrap();

        // unwrap: chat server does not drop our response channel
        res_rx.await.unwrap();
    }
}
