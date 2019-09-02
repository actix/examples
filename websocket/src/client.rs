//! Simple websocket client.
use std::time::Duration;
use std::{io, thread};

use actix::io::SinkWrite;
use actix::*;
use actix_codec::{AsyncRead, AsyncWrite, Framed};
use awc::{
    error::WsProtocolError,
    ws::{Codec, Frame, Message},
    Client,
};
use futures::{
    lazy,
    stream::{SplitSink, Stream},
    Future,
};

fn main() {
    ::std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    let sys = actix::System::new("ws-example");

    Arbiter::spawn(lazy(|| {
        Client::new()
            .ws("http://127.0.0.1:8080/ws/")
            .connect()
            .map_err(|e| {
                println!("Error: {}", e);
            })
            .map(|(response, framed)| {
                println!("{:?}", response);
                let (sink, stream) = framed.split();
                let addr = ChatClient::create(|ctx| {
                    ChatClient::add_stream(stream, ctx);
                    ChatClient(SinkWrite::new(sink, ctx))
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
            })
    }));

    let _ = sys.run();
}

struct ChatClient<T>(SinkWrite<SplitSink<Framed<T, Codec>>>)
where
    T: AsyncRead + AsyncWrite;

#[derive(Message)]
struct ClientCommand(String);

impl<T: 'static> Actor for ChatClient<T>
where
    T: AsyncRead + AsyncWrite,
{
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

impl<T: 'static> ChatClient<T>
where
    T: AsyncRead + AsyncWrite,
{
    fn hb(&self, ctx: &mut Context<Self>) {
        ctx.run_later(Duration::new(1, 0), |act, ctx| {
            act.0.write(Message::Ping(String::new())).unwrap();
            act.hb(ctx);

            // client should also check for a timeout here, similar to the
            // server code
        });
    }
}

/// Handle stdin commands
impl<T: 'static> Handler<ClientCommand> for ChatClient<T>
where
    T: AsyncRead + AsyncWrite,
{
    type Result = ();

    fn handle(&mut self, msg: ClientCommand, _ctx: &mut Context<Self>) {
        self.0.write(Message::Text(msg.0)).unwrap();
    }
}

/// Handle server websocket messages
impl<T: 'static> StreamHandler<Frame, WsProtocolError> for ChatClient<T>
where
    T: AsyncRead + AsyncWrite,
{
    fn handle(&mut self, msg: Frame, _ctx: &mut Context<Self>) {
        if let Frame::Text(txt) = msg {
            println!("Server: {:?}", txt)
        }
    }

    fn started(&mut self, _ctx: &mut Context<Self>) {
        println!("Connected");
    }

    fn finished(&mut self, ctx: &mut Context<Self>) {
        println!("Server disconnected");
        ctx.stop()
    }
}

impl<T: 'static> actix::io::WriteHandler<WsProtocolError> for ChatClient<T> where
    T: AsyncRead + AsyncWrite
{
}
