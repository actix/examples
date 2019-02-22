use actix::{Actor, AsyncContext, Context, Message, StreamHandler};
use bytes::BytesMut;
use futures::stream::SplitSink;
use futures::{Future, Sink, Stream};
use std::net::SocketAddr;
use tokio::codec::BytesCodec;
use tokio::net::{UdpFramed, UdpSocket};

struct UdpActor {
    sink: SplitSink<UdpFramed<BytesCodec>>,
}
impl Actor for UdpActor {
    type Context = Context<Self>;
}

#[derive(Message)]
struct UdpPacket(BytesMut, SocketAddr);
impl StreamHandler<UdpPacket, std::io::Error> for UdpActor {
    fn handle(&mut self, msg: UdpPacket, _: &mut Context<Self>) {
        println!("Received: ({:?}, {:?})", msg.0, msg.1);
        (&mut self.sink).send((msg.0.into(), msg.1)).wait().unwrap();
    }
}

fn main() {
    let sys = actix::System::new("echo-udp");

    let addr: SocketAddr = "127.0.0.1:0".parse().unwrap();
    let sock = UdpSocket::bind(&addr).unwrap();
    println!(
        "Started udp server on: 127.0.0.1:{:?}",
        sock.local_addr().unwrap().port()
    );

    let (sink, stream) = UdpFramed::new(sock, BytesCodec::new()).split();
    UdpActor::create(|ctx| {
        ctx.add_stream(stream.map(|(data, sender)| UdpPacket(data, sender)));
        UdpActor { sink: sink }
    });

    std::process::exit(sys.run());
}
