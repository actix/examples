//! WebSocket connection handling and chat command parsing.
//!
//! Aggregates fragmented messages, checks heartbeats, and forwards chat messages through the actor.
//! Closes connections on shutdown and removes each session from the actor when it ends.

use std::time::{Duration, Instant};

use actix_ws::AggregatedMessage;
use futures_util::StreamExt as _;
use ractor::{ActorProcessingErr, ActorRef};
use tokio::sync::mpsc;
use tokio_util::{future::FutureExt as _, sync::CancellationToken};

use crate::server::Command;

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

/// Handle one WebSocket connection and remove it when the connection ends.
pub async fn chat_ws(
    server: ActorRef<Command>,
    mut session: actix_ws::Session,
    messages: actix_ws::MessageStream,
    shutdown: CancellationToken,
) {
    let (addr, mut inbox) = mpsc::unbounded_channel();
    let Ok(id) = ractor::call!(server, |reply| Command::Connect { addr, reply }) else {
        let _ = session.close(None).await;
        return;
    };

    let mut room = "main".to_owned();
    let mut name = None;
    let mut heartbeat = Instant::now();
    let mut timer = tokio::time::interval(HEARTBEAT_INTERVAL);

    let mut messages = messages
        .max_frame_size(128 * 1024)
        .aggregate_continuations()
        .max_continuation_size(2 * 1024 * 1024);

    let reason = loop {
        tokio::select! {
            _ = shutdown.cancelled() => break Some(actix_ws::CloseCode::Away.into()),

            incoming = messages.next() => {
                match incoming {
                    Some(Ok(AggregatedMessage::Ping(bytes))) => {
                        heartbeat = Instant::now();
                        if session.pong(&bytes).await.is_err() { break None; }
                    }
                    Some(Ok(AggregatedMessage::Pong(_))) => heartbeat = Instant::now(),
                    Some(Ok(AggregatedMessage::Text(text))) => {
                        if process_text(&server, &mut session, id, &mut room, &mut name, &text).await.is_err() {
                            break None;
                        }
                    }
                    Some(Ok(AggregatedMessage::Close(reason))) => break reason,
                    Some(Ok(AggregatedMessage::Binary(_))) => tracing::warn!("Unexpected binary message"),
                    Some(Err(_)) | None => break None,
                }
            }

            outgoing = inbox.recv() => {
                match outgoing {
                    Some(text) => if session.text(text).await.is_err() { break None; },
                    None => break None,
                }
            }

            _ = timer.tick() => {
                if heartbeat.elapsed() > CLIENT_TIMEOUT || session.ping(b"").await.is_err() {
                    break None;
                }
            }
        }
    };

    let _ = server.cast(Command::Disconnect { id });

    // A slow peer must not hold up shutdown while the close frame is queued.
    let _ = session.close(reason).timeout(Duration::from_secs(1)).await;
}

async fn process_text(
    server: &ActorRef<Command>,
    session: &mut actix_ws::Session,
    id: u64,
    room: &mut String,
    name: &mut Option<String>,
    text: &str,
) -> Result<(), ActorProcessingErr> {
    let text = text.trim();

    if text.starts_with('/') {
        let (command, argument) = text
            .split_once(' ')
            .map_or((text, None), |(a, b)| (a, Some(b)));

        match (command, argument) {
            ("/list", _) => {
                for room in ractor::call!(server, Command::ListRooms)? {
                    session.text(room).await?;
                }
            }
            ("/join", Some(next)) => {
                server.cast(Command::Join {
                    id,
                    name: next.to_owned(),
                })?;
                next.clone_into(room);
                session.text("joined").await?;
            }
            ("/join", None) => session.text("!!! room name is required").await?,
            ("/name", Some(next)) => *name = Some(next.to_owned()),
            ("/name", None) => session.text("!!! name is required").await?,
            _ => {
                session
                    .text(format!("!!! unknown command: {text:?}"))
                    .await?
            }
        }
    } else {
        let msg = match name {
            Some(name) => format!("{name}: {text}"),
            None => text.to_owned(),
        };

        server.cast(Command::ClientMessage {
            id,
            msg,
            room: room.clone(),
        })?;
    }

    Ok(())
}
