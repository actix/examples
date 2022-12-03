mod websocket;
mod utf8;

use actix_web::{middleware, web, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("starting HTTP server at http://localhost:9001");

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(web::resource("/").route(web::get().to(websocket::index)))
    })
    .workers(2)
    .bind(("0.0.0.0", 9001))?
    .run()
    .await
}
