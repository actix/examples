//! Simple echo websocket server.
//!
//! Open `http://localhost:8080/` in browser to test.

use actix_files::NamedFile;
use actix_web::{App, Error, HttpRequest, HttpResponse, HttpServer, Responder, middleware, web};
use ractor::Actor;

mod server;
use self::server::{MyWebSocket, WsMessage};

async fn index() -> impl Responder {
    NamedFile::open_async("./static/index.html").await.unwrap()
}

/// WebSocket handshake and start `MyWebSocket` actor.
async fn echo_ws(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let (res, session, stream) = actix_ws::handle(&req, stream)?;

    let (actor, _handle) = Actor::spawn(None, MyWebSocket, session).await.unwrap();

    actix_web::rt::spawn(async move {
        let mut stream = stream.aggregate_continuations();

        while let Some(Ok(msg)) = stream.recv().await {
            actor.send_message(WsMessage::Ws(msg)).unwrap();
        }
    });

    Ok(res)
}

// the actor-based WebSocket examples REQUIRE `actix_web::main` for actor support
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("starting HTTP server at http://localhost:8080");

    HttpServer::new(|| {
        App::new()
            // WebSocket UI HTML file
            .service(web::resource("/").to(index))
            // websocket route
            .service(web::resource("/ws").route(web::get().to(echo_ws)))
            // enable logger
            .wrap(middleware::Logger::default())
    })
    .workers(2)
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
