//! Simple websocket client.

#![allow(unused_variables)]
extern crate actix;
extern crate actix_web;
extern crate env_logger;
extern crate futures;

use std::time::Duration;
use std::{io, thread};

use actix::*;
use actix_web::ws::{Client, ClientWriter, Message, ProtocolError};
use futures::Future;

fn main() {
    ::std::env::set_var("RUST_LOG", "actix_web=info");
    let _ = env_logger::init();
    let sys = actix::System::new("ws-example");

    Arbiter::spawn(
        Client::new("http://127.0.0.1:8080/ws/")
            .connect()
            .map_err(|e| {
                println!("Error: {}", e);
                ()
            })
            .map(|(reader, writer)| {
                let addr = ChatClient::create(|ctx| {
                    ChatClient::add_stream(reader, ctx);
                    ChatClient(writer)
                });

                // start console loop
                thread::spawn(move || loop {
                    let mut cmd = String::new();
                    if io::stdin().read_line(&mut cmd).is_err() {
                        println!("error");
                        return;
                    }
                    addr.do_send(ClientCommand(cmd));
                });

                ()
            }),
    );

    let _ = sys.run();
}

struct ChatClient(ClientWriter);

#[derive(Message)]
struct ClientCommand(String);

impl Actor for ChatClient {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Context<Self>) {
        // start heartbeats otherwise server will disconnect after 10 seconds
        self.hb(ctx)
    }

    fn stopped(&mut self, _: &mut Context<Self>) {
        println!("Disconnected");

        // Stop application on disconnect
        System::current().stop();
    }
}

impl ChatClient {
    fn hb(&self, ctx: &mut Context<Self>) {
        ctx.run_later(Duration::new(1, 0), |act, ctx| {
            act.0.ping("");
            act.hb(ctx);

            // client should also check for a timeout here, similar to the
            // server code
        });
    }
}

/// Handle stdin commands
impl Handler<ClientCommand> for ChatClient {
    type Result = ();

    fn handle(&mut self, msg: ClientCommand, ctx: &mut Context<Self>) {
        self.0.text(msg.0)
    }
}

/// Handle server websocket messages
impl StreamHandler<Message, ProtocolError> for ChatClient {
    fn handle(&mut self, msg: Message, ctx: &mut Context<Self>) {
        match msg {
            Message::Text(txt) => println!("Server: {:?}", txt),
            _ => (),
        }
    }

    fn started(&mut self, ctx: &mut Context<Self>) {
        println!("Connected");
    }

    fn finished(&mut self, ctx: &mut Context<Self>) {
        println!("Server disconnected");
        ctx.stop()
    }
}
