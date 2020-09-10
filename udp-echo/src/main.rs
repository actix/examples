use actix::io::SinkWrite;
use actix::{Actor, AsyncContext, Context, Message, StreamHandler};
use bytes::Bytes;
use bytes::BytesMut;
use futures::stream::SplitSink;
use futures_util::stream::StreamExt;
use std::io::Result;
use std::net::SocketAddr;
use tokio::net::UdpSocket;
use tokio_util::codec::BytesCodec;
use tokio_util::udp::UdpFramed;

type SinkItem = (Bytes, SocketAddr);
type UdpSink = SplitSink<UdpFramed<BytesCodec>, SinkItem>;

struct UdpActor {
    sink: SinkWrite<SinkItem, UdpSink>,
}
impl Actor for UdpActor {
    type Context = Context<Self>;
}

#[derive(Message)]
#[rtype(result = "()")]
struct UdpPacket(BytesMut, SocketAddr);

impl StreamHandler<UdpPacket> for UdpActor {
    fn handle(&mut self, msg: UdpPacket, _: &mut Context<Self>) {
        println!("Received: ({:?}, {:?})", msg.0, msg.1);
        self.sink.write((msg.0.into(), msg.1)).unwrap();
    }
}

impl actix::io::WriteHandler<std::io::Error> for UdpActor {}

#[actix_rt::main]
async fn main() {
    let addr: SocketAddr = "127.0.0.1:0".parse().unwrap();
    let sock = UdpSocket::bind(&addr).await.unwrap();
    println!(
        "Started udp server on: 127.0.0.1:{:?}",
        sock.local_addr().unwrap().port()
    );
    let (sink, stream) = UdpFramed::new(sock, BytesCodec::new()).split();
    UdpActor::create(|ctx| {
        ctx.add_stream(stream.filter_map(
            |item: Result<(BytesMut, SocketAddr)>| async {
                item.map(|(data, sender)| UdpPacket(data, sender)).ok()
            },
        ));
        UdpActor {
            sink: SinkWrite::new(sink, ctx),
        }
    });

    actix_rt::Arbiter::local_join().await;
}
