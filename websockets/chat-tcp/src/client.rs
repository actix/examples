//! Interactive TCP client for the length-prefixed JSON chat protocol.

use std::{io, thread};

use examples_common::init_standard_logger;
use futures_util::{SinkExt as _, StreamExt as _};
use tokio::{net::TcpStream, select, sync::mpsc};
use tokio_stream::wrappers::UnboundedReceiverStream;

mod codec;

#[tokio::main]
async fn main() {
    init_standard_logger();

    println!("Running chat client");

    let (cmd_tx, cmd_rx) = mpsc::unbounded_channel();
    let mut cmd_rx = UnboundedReceiverStream::new(cmd_rx);

    // run blocking terminal input reader on separate thread
    let input_thread = thread::spawn(move || {
        loop {
            let mut cmd = String::with_capacity(32);

            if io::stdin().read_line(&mut cmd).is_err() {
                tracing::error!("Error reading line");
                return;
            }

            if cmd.trim() == "/exit" {
                println!("Exiting input loop");
                return;
            }

            cmd_tx.send(cmd).unwrap();
        }
    });

    let io = TcpStream::connect(("127.0.0.1", 12345)).await.unwrap();
    let mut framed = actix_codec::Framed::new(io, codec::ClientChatCodec);

    loop {
        select! {
            Some(msg) = framed.next() => {
                match msg {
                    Ok(codec::ChatResponse::Message(ref msg)) => {
                        println!("Message: {msg}");
                    }
                    Ok(codec::ChatResponse::Joined(ref msg)) => {
                        println!("!!! Joined: {msg}");
                    }

                    Ok(codec::ChatResponse::Rooms(rooms)) => {
                        println!("!!! Available rooms:");
                        for room in rooms {
                            println!("{room}");
                        }
                    }

                    // respond to pings with a "pong"
                    Ok(codec::ChatResponse::Ping) => { framed.send(codec::ChatRequest::Ping).await.unwrap(); },

                    _ => { eprintln!("{msg:?}"); }
                }
            }

            Some(cmd) = cmd_rx.next() => {
                if cmd.is_empty() {
                    continue;
                }

                if cmd == "/exit" {
                    println!("Exiting recv loop");
                    return;
                }

                if let Some(req) = parse_client_command(&cmd) {
                    // submit client command
                    framed.send(req).await.unwrap();
                }
            }

            else => break
        }
    }

    input_thread.join().unwrap();
}

fn parse_client_command(msg: &str) -> Option<codec::ChatRequest> {
    let m = msg.trim();

    if m.is_empty() {
        return None;
    }

    // we check for /sss type of messages
    if m.starts_with('/') {
        let v: Vec<&str> = m.splitn(2, ' ').collect();
        match v[0] {
            "/list" => Some(codec::ChatRequest::List),
            "/join" => {
                if v.len() == 2 {
                    Some(codec::ChatRequest::Join(v[1].to_owned()))
                } else {
                    println!("!!! Room name is required");
                    None
                }
            }
            _ => {
                println!("!!! Unknown command");
                None
            }
        }
    } else {
        Some(codec::ChatRequest::Message(m.to_owned()))
    }
}
