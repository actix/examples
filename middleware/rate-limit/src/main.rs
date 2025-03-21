use std::io;

use actix_governor::{Governor, GovernorConfigBuilder};
use actix_web::{
    App, HttpResponse, HttpServer, middleware,
    web::{self},
};

mod rate_limit;

async fn index() -> HttpResponse {
    HttpResponse::Ok().body("succeed")
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let governor_config = GovernorConfigBuilder::default()
        .per_second(10)
        .burst_size(2)
        .finish()
        .unwrap();

    log::info!("starting HTTP server at http://localhost:8080");

    HttpServer::new(move || {
        App::new()
            .service(
                web::resource("/test/governor")
                    .wrap(Governor::new(&governor_config))
                    .route(web::get().to(index)),
            )
            .service(
                web::resource("/test/simple")
                    .wrap(rate_limit::RateLimit::new(2))
                    .route(web::get().to(index)),
            )
            .wrap(middleware::NormalizePath::trim())
            .wrap(middleware::Logger::default())
    })
    .workers(2)
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
