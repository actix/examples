use std::{io, thread};

use futures_util::{SinkExt as _, StreamExt as _};
use tokio::{net::TcpStream, select, sync::mpsc};
use tokio_stream::wrappers::UnboundedReceiverStream;

mod codec;

#[actix_web::main]
async fn main() {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    println!("Running chat client");

    let (cmd_tx, cmd_rx) = mpsc::unbounded_channel();
    let mut cmd_rx = UnboundedReceiverStream::new(cmd_rx);

    // run blocking terminal input reader on separate thread
    let input_thread = thread::spawn(move || {
        loop {
            let mut cmd = String::with_capacity(32);

            if io::stdin().read_line(&mut cmd).is_err() {
                log::error!("error reading line");
                return;
            }

            if cmd.trim() == "/exit" {
                println!("exiting input loop");
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
                        println!("message: {msg}");
                    }
                    Ok(codec::ChatResponse::Joined(ref msg)) => {
                        println!("!!! joined: {msg}");
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
                    println!("exiting recv loop");
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
                    println!("!!! room name is required");
                    None
                }
            }
            _ => {
                println!("!!! unknown command");
                None
            }
        }
    } else {
        Some(codec::ChatRequest::Message(m.to_owned()))
    }
}
