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
use ractor::{Actor, ActorRef};

mod server;
mod session;

async fn index() -> impl Responder {
    NamedFile::open("./static/index.html").unwrap()
}

/// Entry point for our websocket route
async fn chat_route(
    req: HttpRequest,
    stream: web::Payload,
    srv: web::Data<ActorRef<server::Command>>,
) -> Result<HttpResponse, Error> {
    let (response, session, messages) = actix_ws::handle(&req, stream)?;

    tokio::task::spawn_local(session::chat_ws(srv.get_ref().clone(), session, messages));

    Ok(response)
}

/// Displays state
async fn get_count(count: web::Data<AtomicUsize>) -> impl Responder {
    let current_count = count.load(Ordering::SeqCst);
    format!("Visitors: {current_count}")
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // set up applications state
    // keep a count of the number of visitors
    let app_state = Arc::new(AtomicUsize::new(0));

    // start chat server actor
    let (server, server_task) =
        server::ChatServer::spawn(None, server::ChatServer, app_state.clone())
            .await
            .map_err(std::io::Error::other)?;
    let shutdown = server.clone();

    log::info!("starting HTTP server at http://localhost:8080");

    let result = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::from(app_state.clone()))
            .app_data(web::Data::new(server.clone()))
            .service(web::resource("/").to(index))
            .route("/count", web::get().to(get_count))
            .route("/ws", web::get().to(chat_route))
            .service(Files::new("/static", "./static"))
            .wrap(Logger::default())
    })
    .workers(2)
    .bind(("127.0.0.1", 8080))?
    .run()
    .await;

    shutdown.stop(None);
    server_task.await.map_err(std::io::Error::other)?;
    result
}
