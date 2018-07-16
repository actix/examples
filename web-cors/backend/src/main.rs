#[macro_use]
extern crate serde_derive;
extern crate actix;
extern crate actix_web;
extern crate env_logger;
extern crate futures;
extern crate serde;
extern crate serde_json;

use actix_web::{
    http::{header, Method},
    middleware,
    middleware::cors::Cors,
    server, App,
};
use std::env;

mod user;
use user::info;

fn main() {
    env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let sys = actix::System::new("Actix-web-CORS");

    server::new(move || {
        App::new()
            .middleware(middleware::Logger::default())
            .configure(|app| {
                Cors::for_app(app)
                    .allowed_origin("http://localhost:1234")
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .max_age(3600)
                    .resource("/user/info", |r| {
                        r.method(Method::POST).with(info);
                    })
                    .register()
            })
    }).bind("127.0.0.1:8000")
        .unwrap()
        .shutdown_timeout(2)
        .start();

    let _ = sys.run();
}
