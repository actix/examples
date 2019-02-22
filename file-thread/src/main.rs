use std::{env, fs, io::Read, process, thread, time};

use actix::{Actor, Context, Handler, Message, Running};
use bytes::BytesMut;

struct FileProcessor {}

impl Actor for FileProcessor {
    type Context = Context<Self>;

    fn started(&mut self, _: &mut Self::Context) {
        println!("file processor started")
    }

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        println!("file processor stopping");
        Running::Stop
    }
}

#[derive(Message, Default)]
struct BytesPacket(BytesMut);

impl Handler<BytesPacket> for FileProcessor {
    type Result = ();

    fn handle(&mut self, msg: BytesPacket, _: &mut Context<Self>) {
        match msg.0.len() {
            0 => {
                println!("eof received, exiting");
                actix::System::current().stop()
            }
            size => println!("packet received: {}", size),
        }
    }
}

fn main() {
    let sys = actix::System::new("file-reader");

    let path = env::args().skip(1).next().expect("please provide filepath");
    let mut file = fs::File::open(path).expect("open failed");
    let addr = FileProcessor {}.start();

    thread::spawn(move || {
        println!("file thread started");
        let mut buffer = [0; 16];
        loop {
            match file.read(&mut buffer) {
                Ok(0) => break,
                Ok(size) => {
                    addr.do_send(BytesPacket(buffer[..size].into()));
                    // following sleep call used as yield for context
                    // switch to actix thread. without it actix
                    // panicked somewhere inside mailbox on big files
                    thread::sleep(time::Duration::from_millis(1))
                }
                Err(err) => panic!("read error: {:?}", err),
            }
        }
        // send empty message to notify about EOF
        addr.do_send(Default::default());
        println!("file thread ended");
    });

    process::exit(sys.run());
}
