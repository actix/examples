//! Multi-room WebSocket chat server.
//!
//! Open `http://localhost:8080/` in browser to test.

use actix_files::NamedFile;
use actix_web::{
    middleware, rt, web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder,
};
use tokio::sync::mpsc::UnboundedSender;

mod command;
mod handler;
mod server;

pub use self::command::Command;
pub use self::server::ChatServer;

/// Connection ID.
pub type ConnId = usize;

/// Room ID.
pub type RoomId = String;

/// Message sent to a room/client.
pub type Msg = String;

async fn index() -> impl Responder {
    NamedFile::open_async("./static/index.html").await.unwrap()
}

/// Handshake and start WebSocket handler with heartbeats.
async fn chat_ws(
    req: HttpRequest,
    stream: web::Payload,
    server_tx: web::Data<UnboundedSender<Command>>,
) -> Result<HttpResponse, Error> {
    let (res, session, msg_stream) = actix_ws::handle(&req, stream)?;

    // spawn websocket handler (and don't await it) so that the response is returned immediately
    rt::spawn(handler::chat_ws((**server_tx).clone(), session, msg_stream));

    Ok(res)
}

// note that the `actix` based WebSocket handling would NOT work under `tokio::main`
#[tokio::main(flavor = "current_thread")]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("starting HTTP server at http://localhost:8080");

    let (chat_server, server_tx) = ChatServer::new();

    rt::spawn(chat_server.run());

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(server_tx.clone()))
            // WebSocket UI HTML file
            .service(web::resource("/").to(index))
            // websocket routes
            .service(web::resource("/ws").route(web::get().to(chat_ws)))
            // enable logger
            .wrap(middleware::Logger::default())
    })
    .workers(2)
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
