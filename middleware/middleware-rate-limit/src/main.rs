use std::io;

use actix_governor::{Governor, GovernorConfigBuilder};
use actix_web::{
    middleware,
    web::{self},
    App, HttpResponse, HttpServer,
};

async fn index() -> HttpResponse {
    HttpResponse::Ok().body("succeed")
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let limit_cfg = GovernorConfigBuilder::default()
        .per_second(10)
        .burst_size(2)
        .finish()
        .unwrap();

    log::info!("starting HTTP server at http://localhost:8080");

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(Governor::new(&limit_cfg))
            .service(web::resource("/test").route(web::get().to(index)))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
