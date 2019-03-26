#[macro_use]
extern crate actix_web;

use std::collections::HashMap;

use actix_web::{
    error::ErrorInternalServerError, middleware, web::Query, App, HttpResponse,
    HttpServer, Result,
};
use yarte::Template;

#[derive(Template)]
#[template(path = "index.hbs")]
struct IndexTemplate {
    query: Query<HashMap<String, String>>,
}

#[get("/")]
pub fn index(query: Query<HashMap<String, String>>) -> Result<HttpResponse> {
    IndexTemplate { query }
        .call()
        .map(|s| {
            HttpResponse::Ok()
                .content_type(IndexTemplate::mime())
                .body(s)
        })
        .map_err(|_| ErrorInternalServerError("Template parsing error"))
}

fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    // start http server
    HttpServer::new(|| {
        App::new()
            // enable logger
            .wrap(middleware::Logger::default())
            .service(index)
    })
    .bind("127.0.0.1:8080")?
    .run()
}
