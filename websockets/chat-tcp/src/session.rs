//! TCP listener and connection handling.
//!
//! Decodes chat commands, forwards actor messages, and checks client heartbeats.

use std::{
    io,
    time::{Duration, Instant},
};

use err_report::Report;
use futures_util::{SinkExt as _, StreamExt as _};
use ractor::{ActorProcessingErr, ActorRef};
use tokio::{
    net::{TcpListener, TcpStream},
    sync::mpsc,
    task::JoinSet,
};
use tokio_util::{codec::Framed, future::FutureExt as _, sync::CancellationToken};

use crate::{
    codec::{ChatCodec, ChatRequest, ChatResponse},
    server::Command,
};

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(1);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

/// Accept TCP connections until shutdown, then wait for their tasks to finish.
pub async fn tcp_server(
    listener: TcpListener,
    server: ActorRef<Command>,
    shutdown: CancellationToken,
) -> io::Result<()> {
    // Stop the HTTP server too if the TCP listener fails.
    let _guard = shutdown.clone().drop_guard();
    let mut sessions = JoinSet::new();

    let result = loop {
        tokio::select! {
            _ = shutdown.cancelled() => break Ok(()),
            connection = listener.accept() => {
                match connection {
                    Ok((stream, _)) => {
                        sessions.spawn(chat_tcp(stream, server.clone(), shutdown.clone()));
                    }
                    Err(err) => break Err(err),
                }
            }
            Some(result) = sessions.join_next() => {
                if let Err(err) = result {
                    tracing::error!(error = %Report::new(&err), "TCP session task failed");
                }
            }
        }
    };

    drop(listener);
    shutdown.cancel();

    while let Some(result) = sessions.join_next().await {
        if let Err(err) = result {
            tracing::error!(error = %Report::new(&err), "TCP session task failed");
        }
    }

    result
}

/// Handle a TCP connection and remove its registration when it closes.
async fn chat_tcp(stream: TcpStream, server: ActorRef<Command>, shutdown: CancellationToken) {
    let (addr, mut inbox) = mpsc::unbounded_channel();
    let Ok(id) = ractor::call!(server, |reply| Command::Connect { addr, reply }) else {
        return;
    };

    let mut framed = Framed::new(stream, ChatCodec);
    let mut room = "main".to_owned();
    let mut heartbeat = Instant::now();
    let mut timer = tokio::time::interval(HEARTBEAT_INTERVAL);

    let result: Result<(), ActorProcessingErr> = async {
        loop {
            tokio::select! {
                _ = shutdown.cancelled() => break,
                request = framed.next() => {
                    match request {
                        Some(Ok(ChatRequest::List)) => {
                            let rooms = ractor::call!(server, Command::ListRooms)?;
                            send(&mut framed, ChatResponse::Rooms(rooms)).await?;
                        }
                        Some(Ok(ChatRequest::Join(name))) => {
                            server.cast(Command::Join { id, name: name.clone() })?;
                            name.clone_into(&mut room);
                            send(&mut framed, ChatResponse::Joined(name)).await?;
                        }
                        Some(Ok(ChatRequest::Message(msg))) => {
                            server.cast(Command::ClientMessage { id, msg, room: room.clone() })?;
                        }
                        Some(Ok(ChatRequest::Ping)) => heartbeat = Instant::now(),
                        Some(Err(err)) => return Err(err.into()),
                        None => break,
                    }
                }
                message = inbox.recv() => {
                    match message {
                        Some(message) => send(&mut framed, ChatResponse::Message(message)).await?,
                        None => break,
                    }
                }
                _ = timer.tick() => {
                    if heartbeat.elapsed() > CLIENT_TIMEOUT {
                        break;
                    }

                    send(&mut framed, ChatResponse::Ping).await?;
                }
            }
        }

        Ok(())
    }
    .await;

    let _ = server.cast(Command::Disconnect { id });

    if let Err(err) = result {
        tracing::debug!(error = %Report::new(err.as_ref()), "TCP connection closed");
    }
}

/// Bound writes so a slow client cannot hold up shutdown.
async fn send(framed: &mut Framed<TcpStream, ChatCodec>, message: ChatResponse) -> io::Result<()> {
    framed.send(message).timeout(Duration::from_secs(1)).await?
}
