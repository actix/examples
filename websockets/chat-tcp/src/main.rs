//! TCP and WebSocket chat with a shared Ractor actor.
//!
//! Starts both listeners and stops the actor after connection shutdown.

use actix_files::NamedFile;
use actix_web::{
    App, Error, HttpRequest, HttpResponse, HttpServer, Responder, middleware::Logger, web,
};
use examples_common::{init_standard_logger, watch_shutdown_signal};
use eyre::WrapErr as _;
use ractor::{Actor, ActorRef};
use tokio_util::sync::CancellationToken;

mod codec;
mod server;
mod session;
mod websocket;

async fn index() -> impl Responder {
    NamedFile::open("./static/index.html").unwrap()
}

/// Entry point for our websocket route
async fn chat_route(
    req: HttpRequest,
    stream: web::Payload,
    web::ThinData(srv): web::ThinData<ActorRef<server::Command>>,
    web::ThinData(shutdown): web::ThinData<CancellationToken>,
) -> Result<HttpResponse, Error> {
    let (response, session, messages) = actix_ws::handle(&req, stream)?;

    tokio::task::spawn_local(websocket::chat_ws(srv, session, messages, shutdown));

    Ok(response)
}

#[tokio::main]
async fn main() -> eyre::Result<()> {
    color_eyre::install()?;
    init_standard_logger();

    let listener = tokio::net::TcpListener::bind(("127.0.0.1", 12345))
        .await
        .wrap_err("Failed to bind TCP listener")?;

    // start chat server actor
    let (server, server_task) = server::ChatServer::spawn(None, server::ChatServer, ())
        .await
        .wrap_err("Failed to start chat actor")?;
    let actor = server.clone();
    let shutdown = CancellationToken::new();
    watch_shutdown_signal(shutdown.clone());
    let worker_shutdown = shutdown.clone();
    let tcp_task = tokio::spawn(session::tcp_server(
        listener,
        server.clone(),
        shutdown.clone(),
    ));

    tracing::info!("Starting HTTP server at http://localhost:8080");

    let http_server = HttpServer::new(move || {
        App::new()
            .app_data(web::ThinData(server.clone()))
            .app_data(web::ThinData(worker_shutdown.clone()))
            .service(web::resource("/").to(index))
            .route("/ws", web::get().to(chat_route))
            .wrap(Logger::default())
    })
    .shutdown_signal(shutdown.clone().cancelled_owned())
    .workers(2)
    .bind(("127.0.0.1", 8080));

    let result = match http_server {
        Ok(http_server) => http_server.run().await.wrap_err("HTTP server failed"),
        Err(err) => Err(err).wrap_err("Failed to bind HTTP server"),
    };

    shutdown.cancel();
    let tcp_result = tcp_task.await;
    actor.stop(None);
    server_task.await.wrap_err("Chat actor task failed")?;

    tcp_result
        .wrap_err("TCP listener task failed")?
        .wrap_err("TCP listener failed")?;

    result
}
