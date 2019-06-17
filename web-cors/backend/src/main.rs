#[macro_use]
extern crate serde_derive;

use actix_cors::Cors;
use actix_web::{http::header, middleware::Logger, web, App, HttpServer};

mod user;

fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::new()
                    .allowed_origin("http://localhost:8080")
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .max_age(3600),
            )
            .wrap(Logger::default())
            .service(web::resource("/user/info").route(web::post().to(user::info)))
    })
    .bind("127.0.0.1:8000")?
    .run()
}
