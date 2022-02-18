use actix_files::{Files, NamedFile};
use actix_web::{middleware::Logger, web, App, Error, HttpRequest, HttpServer, Responder};
use actix_web_actors::ws;

mod message;
mod server;
mod session;

use session::WsChatSession;

async fn index() -> impl Responder {
    NamedFile::open_async("./static/index.html").await.unwrap()
}

async fn chat_ws(req: HttpRequest, stream: web::Payload) -> Result<impl Responder, Error> {
    ws::start(WsChatSession::default(), &req, stream)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("starting HTTP server at http://localhost:8080");

    HttpServer::new(move || {
        App::new()
            .service(web::resource("/").to(index))
            .service(web::resource("/ws").to(chat_ws))
            .service(Files::new("/static", "./static"))
            .wrap(Logger::default())
    })
    .workers(2)
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
