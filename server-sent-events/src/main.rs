use actix_web::error::ErrorInternalServerError;
use actix_web::web::{Bytes, Data, Path};
use actix_web::{web, App, Error, HttpResponse, HttpServer, Responder};

use tokio::prelude::*;
use tokio::sync::mpsc::{channel, Receiver, Sender};

use std::sync::Mutex;

fn main() {
    let data = Data::new(Mutex::new(Broadcaster::new()));

    HttpServer::new(move || {
        App::new()
            .register_data(data.clone())
            .route("/", web::get().to(index))
            .route("/events", web::get().to(new_client))
            .route("/broadcast/{msg}", web::get().to(broadcast))
    })
    .bind("127.0.0.1:8080")
    .expect("Unable to bind port")
    .run()
    .unwrap();
}

fn index() -> impl Responder {
    let content = include_str!("index.html");

    HttpResponse::Ok()
        .header("content-type", "text/html")
        .body(content)
}

fn new_client(broadcaster: Data<Mutex<Broadcaster>>) -> impl Responder {
    let rx = broadcaster.lock().unwrap().new_client();

    HttpResponse::Ok()
        .header("content-type", "text/event-stream")
        .no_chunking()
        .streaming(rx)
}

fn broadcast(msg: Path<String>, broadcaster: Data<Mutex<Broadcaster>>) -> impl Responder {
    broadcaster.lock().unwrap().send(&msg.into_inner());

    HttpResponse::Ok().body("msg sent")
}

struct Broadcaster {
    clients: Vec<Sender<Bytes>>,
}

impl Broadcaster {
    fn new() -> Self {
        Broadcaster {
            clients: Vec::new(),
        }
    }

    fn new_client(&mut self) -> Client {
        let (tx, rx) = channel(100);

        tx.clone()
            .try_send(Bytes::from("data: connected\n\n"))
            .unwrap();

        self.clients.push(tx);
        Client(rx)
    }

    fn send(&mut self, msg: &str) {
        let msg = Bytes::from(["data: ", msg, "\n\n"].concat());

        let mut ok_clients = Vec::new();
        for client in self.clients.iter() {
            let result = client.clone().try_send(msg.clone());

            if let Ok(()) = result {
                ok_clients.push(client.clone());
            }
        }

        self.clients = ok_clients;
    }
}

// wrap Receiver in own type, with correct error type
struct Client(Receiver<Bytes>);

impl Stream for Client {
    type Item = Bytes;
    type Error = Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        self.0.poll().map_err(ErrorInternalServerError)
    }
}
