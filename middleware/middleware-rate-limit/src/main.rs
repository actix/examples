use std::io;

use actix_governor::{Governor, GovernorConfigBuilder};
use actix_web::{
    middleware,
    web::{self},
    App, HttpResponse, HttpServer,
};

mod rate_limit;

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
            .service(
                web::resource("/test/governor")
                    .wrap(Governor::new(&limit_cfg))
                    .route(web::get().to(index)),
            )
            .service(
                web::resource("/test/simple")
                    .wrap(rate_limit::RateLimit::new(2))
                    .route(web::get().to(index)),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
