//! Start two `basic` instances
//! 1. cargo run --example basic -- 127.0.0.1:7654
//! 2. ./target/debug/examples/basic 127.0.0.1:7655 127.0.0.1:7654
//!
//! first instance sends messages, second instance respondes to messages from first instance
//!
#![allow(dead_code, unused_variables)]

extern crate log;
extern crate env_logger;
extern crate futures;
#[macro_use] extern crate actix;
extern crate actix_remote;
extern crate serde_json;
#[macro_use] extern crate serde_derive;
extern crate structopt;
#[macro_use] extern crate structopt_derive;

use std::time::Duration;

use actix_remote::*;
use actix::prelude::*;
use futures::Future;
use structopt::StructOpt;

mod msgs;
use msgs::TestMessage;


struct MyActor {
    cnt: usize,
    hb: bool,
    recipient: Recipient<Remote, TestMessage>,
}

impl MyActor {
    fn hb(&self, ctx: &mut Context<Self>) {
        self.recipient.send(TestMessage{msg: "TEST".to_owned()})
            .and_then(|r| {
                println!("REMOTE RESULT: {:?}", r);
                Ok(())
            })
            .map_err(|_| ())
            .into_actor(self)
            .spawn(ctx);

        ctx.run_later(Duration::from_secs(3), |act, ctx| act.hb(ctx));
    }
}

impl Actor for MyActor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Context<Self>) {
        if self.hb {
            self.hb(ctx);
        }
    }
}

impl Handler<TestMessage> for MyActor {
    type Result = ();

    fn handle(&mut self, msg: TestMessage, _ctx: &mut Context<Self>) {
        println!("REMOTE MESSAGE: {:?}", msg);
    }
}

#[derive(StructOpt, Debug)]
struct Cli {
    /// Network address
    addr: String,
    /// Network node address
    node: Option<String>,
}

fn main() {
    ::std::env::set_var("RUST_LOG", "actix_remote=debug");
    let _ = env_logger::init();

    // cmd arguments
    let args = Cli::from_args();
    let addr = args.addr.to_lowercase().trim().to_owned();
    let node = args.node.map(|n| n.to_lowercase().trim().to_owned());

    let sys = actix::System::new("remote-example");

    // send messages from main instance
    let hb = node.is_none();

    // create world
    let mut world = World::new(addr).unwrap().add_node(node);

    // get remote recipient
    let recipient = world.get_recipient::<TestMessage>();

    let addr = world.start();
    let a: Addr<Unsync, _> = MyActor::create(move |ctx| {
        ctx.run_later(Duration::from_millis(5000), move |_, ctx| {
            // register actor as recipient for `TestMessage` message
            World::register_recipient(
                &addr, ctx.address::<Addr<Syn, _>>().recipient());
        });

        MyActor{cnt: 0, hb, recipient}
    });

    let _ = sys.run();
}