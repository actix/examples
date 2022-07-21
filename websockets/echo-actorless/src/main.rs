//! Simple echo websocket server.
//!
//! Open `http://localhost:8080/` in browser to test.

use actix_files::NamedFile;
use actix_web::{
    middleware, rt, web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder,
};

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
/// This example is just for demonstration of simplicity. In reality, you likely want to include
/// some handling of heartbeats for connection health tracking to free up server resources when
/// connections die or network issues arise.
async fn echo_ws(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let (res, session, msg_stream) = actix_ws::handle(&req, stream)?;

    // spawn websocket handler (and don't await it) so that the response is returned immediately
    rt::spawn(handler::echo_ws(session, msg_stream));

    Ok(res)
}

// note that the `actix` based WebSocket handling would NOT work under `tokio::main`
#[tokio::main(flavor = "current_thread")]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("starting HTTP server at http://localhost:8080");

    HttpServer::new(|| {
        App::new()
            // WebSocket UI HTML file
            .service(web::resource("/").to(index))
            // websocket routes
            .service(web::resource("/ws").route(web::get().to(echo_heartbeat_ws)))
            .service(web::resource("/ws-basic").route(web::get().to(echo_ws)))
            // enable logger
            .wrap(middleware::Logger::default())
    })
    .workers(2)
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
