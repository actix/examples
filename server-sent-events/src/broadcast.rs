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
use futures_util::Stream;
use parking_lot::Mutex;
use tokio::sync::mpsc::{channel, Sender, Receiver};

pub struct Broadcaster {
    inner: Mutex<BroadcasterInner>,
}

struct BroadcasterInner {
    clients: Vec<Sender<Bytes>>,
}

impl Broadcaster {
    pub fn create() -> Data<Self> {
        // Data ~≃ Arc
        let me = Data::new(Broadcaster {
            inner: Mutex::new(BroadcasterInner {
                clients: Vec::new(),
            }),
        });

        // ping clients every 10 seconds to see if they are alive
        Broadcaster::spawn_ping(me.clone());

        me
    }

    fn spawn_ping(me: Data<Self>) {
        actix_web::rt::spawn(async move {
            let mut interval = interval_at(Instant::now(), Duration::from_secs(10));

            loop {
                interval.tick().await;
                me.remove_stale_clients();
            }
        });
    }

    fn remove_stale_clients(&self) {
        let mut inner = self.inner.lock();

        let mut ok_clients = Vec::new();
        for client in inner.clients.iter() {
            let result = client.clone().try_send(Bytes::from("data: ping\n\n"));

            if let Ok(()) = result {
                ok_clients.push(client.clone());
            }
        }
        inner.clients = ok_clients;
    }

    pub fn new_client(&self) -> Client {
        let (tx, rx) = channel(100);

        tx.try_send(Bytes::from("data: connected\n\n")).unwrap();

        let mut inner = self.inner.lock();
        inner.clients.push(tx);

        Client(rx)
    }

    pub fn send(&self, msg: &str) {
        let msg = Bytes::from(["data: ", msg, "\n\n"].concat());

        let inner = self.inner.lock();
        for client in inner.clients.iter() {
            client.clone().try_send(msg.clone()).unwrap_or(());
        }
    }
}

// wrap Receiver in own type, with correct error type
pub struct Client(Receiver<Bytes>);

impl Stream for Client {
    type Item = Result<Bytes, Error>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        match Pin::new(&mut self.0).poll_recv(cx) {
            Poll::Ready(Some(v)) => Poll::Ready(Some(Ok(v))),
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Pending => Poll::Pending,
        }
    }
}
