use std::time::{Duration, Instant};

use actix_web::rt;
use actix_ws::Message;
use futures_util::stream::StreamExt as _;
use tokio::{
    select,
    sync::{
        mpsc::{self, UnboundedSender},
        oneshot,
    },
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
    let mut interval = rt::time::interval(HEARTBEAT_INTERVAL);

    let (conn_tx, mut conn_rx) = mpsc::unbounded_channel();
    let (res_tx, res_rx) = oneshot::channel();

    server_tx
        .send(Command::Connect { conn_tx, res_tx })
        .unwrap();

    let conn_id = res_rx.await.unwrap();

    loop {
        select! {
            // commands & messages received from client
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
                        process_text_msg(&server_tx, &mut session, &text, conn_id, &mut room, &mut name).await;
                    }

                    Message::Binary(_bin) => {
                        log::warn!("unexpected binary message");
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

            // chat messages received from other room participants
            Some(chat_msg) = conn_rx.recv() => {
                session.text(chat_msg).await.unwrap();
            }

            // heartbeat
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

async fn process_text_msg(
    server_tx: &UnboundedSender<Command>,
    session: &mut actix_ws::Session,
    text: &str,
    conn: ConnId,
    room: &mut RoomId,
    name: &mut Option<String>,
) {
    let msg = text.trim();

    // we check for /<cmd> type of messages
    if msg.starts_with('/') {
        let mut cmd_args = msg.splitn(2, ' ');

        match cmd_args.next().unwrap() {
            "/list" => {
                // Send ListRooms message to chat server and wait for
                // response
                log::info!("List rooms");

                let (res_tx, res_rx) = oneshot::channel();
                server_tx.send(Command::List { res_tx }).unwrap();
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
            .unwrap();

        res_rx.await.unwrap();
    }
}
