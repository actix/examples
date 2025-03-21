//! Simple echo websocket server.
//!
//! Open `http://localhost:8080/` in browser to test.

use std::io;

use actix_files::NamedFile;
use actix_web::{
    App, Error, HttpRequest, HttpResponse, HttpServer, Responder, middleware, rt, web,
};
use tokio::sync::broadcast;

mod handler;

async fn index() -> impl Responder {
    NamedFile::open_async("./static/index.html").await.unwrap()
}

/// Handshake and start WebSocket handler with heartbeats.
async fn echo_heartbeat_ws(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let (res, session, msg_stream) = actix_ws::handle(&req, stream)?;

    // spawn websocket handler (and don't await it) so that the response is returned immediately
    rt::spawn(handler::echo_heartbeat_ws(session, msg_stream));

    Ok(res)
}

/// Handshake and start basic WebSocket handler.
///
/// This example is just for simple demonstration. In reality, you likely want to include
/// some handling of heartbeats for connection health tracking to free up server resources when
/// connections die or network issues arise.
async fn echo_ws(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let (res, session, msg_stream) = actix_ws::handle(&req, stream)?;

    // spawn websocket handler (and don't await it) so that the response is returned immediately
    rt::spawn(handler::echo_ws(session, msg_stream));

    Ok(res)
}

/// Send message to clients connected to broadcast WebSocket.
async fn send_to_broadcast_ws(
    body: web::Bytes,
    tx: web::Data<broadcast::Sender<web::Bytes>>,
) -> Result<impl Responder, Error> {
    tx.send(body)
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::NoContent())
}

/// Handshake and start broadcast WebSocket handler with heartbeats.
async fn broadcast_ws(
    req: HttpRequest,
    stream: web::Payload,
    tx: web::Data<broadcast::Sender<web::Bytes>>,
) -> Result<HttpResponse, Error> {
    let (res, session, msg_stream) = actix_ws::handle(&req, stream)?;

    // spawn websocket handler (and don't await it) so that the response is returned immediately
    rt::spawn(handler::broadcast_ws(session, msg_stream, tx.subscribe()));

    Ok(res)
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("starting HTTP server at http://localhost:8080");

    let (tx, _) = broadcast::channel::<web::Bytes>(128);

    HttpServer::new(move || {
        App::new()
            // WebSocket UI HTML file
            .service(web::resource("/").to(index))
            // websocket routes
            .service(web::resource("/ws").route(web::get().to(echo_heartbeat_ws)))
            .service(web::resource("/ws-basic").route(web::get().to(echo_ws)))
            .app_data(web::Data::new(tx.clone()))
            .service(web::resource("/ws-broadcast").route(web::get().to(broadcast_ws)))
            .service(web::resource("/send").route(web::post().to(send_to_broadcast_ws)))
            // standard middleware
            .wrap(middleware::NormalizePath::trim())
            .wrap(middleware::Logger::default())
    })
    .workers(2)
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
