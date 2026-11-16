//! Multi-room WebSocket chat server.
//!
//! Open `http://localhost:8080/` in browser to test.

use std::io;

use actix_files::NamedFile;
use actix_web::{App, Error, HttpResponse, HttpServer, Responder, middleware, web};
use actix_ws::WebSocket;
use tokio::{
    task::{spawn, spawn_local},
    try_join,
};

mod handler;
mod server;

pub use self::server::{ChatServer, ChatServerHandle};

/// Connection ID.
pub type ConnId = u64;

/// Room ID.
pub type RoomId = String;

/// Message sent to a room/client.
pub type Msg = String;

async fn index() -> impl Responder {
    NamedFile::open("./static/index.html").unwrap()
}

/// Handshake and start WebSocket handler with heartbeats.
async fn chat_ws(
    WebSocket {
        response,
        session,
        stream,
    }: WebSocket,
    chat_server: web::Data<ChatServerHandle>,
) -> Result<HttpResponse, Error> {
    // spawn websocket handler (and don't await it) so that the response is returned immediately
    spawn_local(handler::chat_ws((**chat_server).clone(), session, stream));

    Ok(response)
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("Starting HTTP server at http://localhost:8080");

    let (chat_server, server_tx) = ChatServer::new();

    let chat_server = spawn(chat_server.run());

    let http_server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(server_tx.clone()))
            // WebSocket UI HTML file
            .service(web::resource("/").to(index))
            // websocket routes
            .service(web::resource("/ws").route(web::get().to(chat_ws)))
            // standard middleware
            .wrap(middleware::NormalizePath::trim())
            .wrap(middleware::Logger::default())
    })
    .workers(2)
    .bind(("127.0.0.1", 8080))?
    .run();

    try_join!(http_server, async move { chat_server.await.unwrap() })?;

    Ok(())
}
