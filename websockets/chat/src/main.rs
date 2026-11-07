//! WebSocket chat example with a shared Ractor chat server.
//!
//! Starts the actor and HTTP server, serves the browser client, and exposes the visitor count.

use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};

use actix_files::{Files, NamedFile};
use actix_web::{
    App, Error, HttpRequest, HttpResponse, HttpServer, Responder, middleware::Logger, web,
};
use examples_common::{init_standard_logger, watch_shutdown_signal};
use eyre::WrapErr as _;
use ractor::{Actor, ActorRef};
use tokio_util::sync::CancellationToken;

mod server;
mod session;

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

    tokio::task::spawn_local(session::chat_ws(srv, session, messages, shutdown));

    Ok(response)
}

/// Displays state
async fn get_count(count: web::Data<AtomicUsize>) -> impl Responder {
    let current_count = count.load(Ordering::SeqCst);
    format!("Visitors: {current_count}")
}

#[tokio::main]
async fn main() -> eyre::Result<()> {
    color_eyre::install()?;
    init_standard_logger();

    // set up applications state
    // keep a count of the number of visitors
    let app_state = Arc::new(AtomicUsize::new(0));

    // start chat server actor
    let (server, server_task) =
        server::ChatServer::spawn(None, server::ChatServer, app_state.clone())
            .await
            .wrap_err("failed to start chat actor")?;
    let actor = server.clone();
    let shutdown = CancellationToken::new();
    watch_shutdown_signal(shutdown.clone());
    let worker_shutdown = shutdown.clone();

    tracing::info!("starting HTTP server at http://localhost:8080");

    let http_server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::from(app_state.clone()))
            .app_data(web::ThinData(server.clone()))
            .app_data(web::ThinData(worker_shutdown.clone()))
            .service(web::resource("/").to(index))
            .route("/count", web::get().to(get_count))
            .route("/ws", web::get().to(chat_route))
            .service(Files::new("/static", "./static"))
            .wrap(Logger::default())
    })
    .shutdown_signal(shutdown.clone().cancelled_owned())
    .workers(2)
    .bind(("127.0.0.1", 8080));

    let result = match http_server {
        Ok(http_server) => http_server.run().await.wrap_err("HTTP server failed"),
        Err(err) => Err(err).wrap_err("failed to bind HTTP server"),
    };

    shutdown.cancel();
    actor.stop(None);
    server_task.await.wrap_err("chat actor task failed")?;

    result
}
