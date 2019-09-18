use actix_web::{middleware, App, HttpServer};

use async_ex2::appconfig::config_app;

fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .configure(config_app)
            .wrap(middleware::Logger::default())
    })
    .bind("127.0.0.1:8080")?
    .run()
}
