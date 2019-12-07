use std::pin::Pin;
use std::sync::Mutex;
use std::task::{Context, Poll};
use std::time::Duration;

use actix_web::web::{Bytes, Data, Path};
use actix_web::{web, App, Error, HttpResponse, HttpServer, Responder};
use env_logger;
use futures::{Stream, StreamExt};
use tokio::sync::mpsc::{channel, Receiver, Sender};
use tokio::time::{interval_at, Instant};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    let data = Broadcaster::create();

    HttpServer::new(move || {
        App::new()
            .register_data(data.clone())
            .route("/", web::get().to(index))
            .route("/events", web::get().to(new_client))
            .route("/broadcast/{msg}", web::get().to(broadcast))
    })
    .bind("127.0.0.1:8080")?
    .start()
    .await
}

async fn index() -> impl Responder {
    let content = include_str!("index.html");

    HttpResponse::Ok()
        .header("content-type", "text/html")
        .body(content)
}

async fn new_client(broadcaster: Data<Mutex<Broadcaster>>) -> impl Responder {
    let rx = broadcaster.lock().unwrap().new_client();

    HttpResponse::Ok()
        .header("content-type", "text/event-stream")
        .no_chunking()
        .streaming(rx)
}

async fn broadcast(
    msg: Path<String>,
    broadcaster: Data<Mutex<Broadcaster>>,
) -> impl Responder {
    broadcaster.lock().unwrap().send(&msg.into_inner());

    HttpResponse::Ok().body("msg sent")
}

struct Broadcaster {
    clients: Vec<Sender<Bytes>>,
}

impl Broadcaster {
    fn create() -> Data<Mutex<Self>> {
        // Data ≃ Arc
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
        actix_rt::spawn(async move {
            let mut task = interval_at(Instant::now(), Duration::from_secs(10));
            while let Some(_) = task.next().await {
                me.lock().unwrap().remove_stale_clients();
            }
        })
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

    fn new_client(&mut self) -> Client {
        let (tx, rx) = channel(100);

        tx.clone()
            .try_send(Bytes::from("data: connected\n\n"))
            .unwrap();

        self.clients.push(tx);
        Client(rx)
    }

    fn send(&self, msg: &str) {
        let msg = Bytes::from(["data: ", msg, "\n\n"].concat());

        for client in self.clients.iter() {
            client.clone().try_send(msg.clone()).unwrap_or(());
        }
    }
}

// wrap Receiver in own type, with correct error type
struct Client(Receiver<Bytes>);

impl Stream for Client {
    type Item = Result<Bytes, Error>;

    fn poll_next(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        match Pin::new(&mut self.0).poll_next(cx) {
            Poll::Ready(Some(v)) => Poll::Ready(Some(Ok(v))),
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Pending => Poll::Pending,
        }
    }
}
