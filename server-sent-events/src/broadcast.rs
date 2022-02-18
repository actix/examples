use std::{
    pin::Pin,
    task::{Context, Poll},
    time::Duration,
};

use actix_web::{
    rt::time::{interval_at, Instant},
    web::{Bytes, Data},
    Error,
};
use futures_util::{Stream, StreamExt as _};
use parking_lot::Mutex;
use tokio::sync::mpsc::{channel, Sender};
use tokio_stream::wrappers::{IntervalStream, ReceiverStream};

pub struct Broadcaster {
    clients: Vec<Sender<Bytes>>,
}

impl Broadcaster {
    pub fn create() -> Data<Mutex<Self>> {
        // Data ~≃ Arc
        let me = Data::new(Mutex::new(Broadcaster::new()));

        // ping clients every 10 seconds to see if they are alive
        Broadcaster::spawn_ping(me.clone());

        me
    }

    fn new() -> Self {
        Broadcaster {
            clients: Vec::new(),
        }
    }

    fn spawn_ping(me: Data<Mutex<Self>>) {
        actix_web::rt::spawn(async move {
            let mut task =
                IntervalStream::new(interval_at(Instant::now(), Duration::from_secs(10)));

            while task.next().await.is_some() {
                me.lock().remove_stale_clients();
            }
        });
    }

    fn remove_stale_clients(&mut self) {
        let mut ok_clients = Vec::new();
        for client in self.clients.iter() {
            let result = client.clone().try_send(Bytes::from("data: ping\n\n"));

            if let Ok(()) = result {
                ok_clients.push(client.clone());
            }
        }
        self.clients = ok_clients;
    }

    pub fn new_client(&mut self) -> Client {
        let (tx, rx) = channel(100);
        let rx = ReceiverStream::new(rx);

        tx.try_send(Bytes::from("data: connected\n\n")).unwrap();

        self.clients.push(tx);
        Client(rx)
    }

    pub fn send(&self, msg: &str) {
        let msg = Bytes::from(["data: ", msg, "\n\n"].concat());

        for client in self.clients.iter() {
            client.clone().try_send(msg.clone()).unwrap_or(());
        }
    }
}

// wrap Receiver in own type, with correct error type
pub struct Client(ReceiverStream<Bytes>);

impl Stream for Client {
    type Item = Result<Bytes, Error>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        match Pin::new(&mut self.0).poll_next(cx) {
            Poll::Ready(Some(v)) => Poll::Ready(Some(Ok(v))),
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Pending => Poll::Pending,
        }
    }
}
