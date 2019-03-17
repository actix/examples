use actix_web::{middleware, web, App, HttpServer};

#[path = "lib.rs"]
mod template;

fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    // start http server
    HttpServer::new(|| {
        App::new()
            // enable logger
            .middleware(middleware::Logger::default())
            .service(web::resource("/").to(template::index))
    })
    .bind("127.0.0.1:8080")?
    .run()
}
