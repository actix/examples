#![allow(clippy::type_complexity)]

use actix_service::Service;
use actix_web::{web, App, HttpServer};
use futures::future::FutureExt;

#[allow(dead_code)]
mod read_request_body;
#[allow(dead_code)]
mod read_response_body;
#[allow(dead_code)]
mod redirect;
#[allow(dead_code)]
mod simple;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(redirect::CheckLogin)
            .wrap(read_request_body::Logging)
            .wrap(read_response_body::Logging)
            .wrap(simple::SayHi)
            .wrap_fn(|req, srv| {
                println!("Hi from start. You requested: {}", req.path());

                srv.call(req).map(|res| {
                    println!("Hi from response");
                    res
                })
            })
            .service(web::resource("/login").to(|| async {
                "You are on /login. Go to src/redirect.rs to change this behavior."
            }))
            .service(web::resource("/").to(|| async {
                "Hello, middleware! Check the console where the server is run."
            }))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
