extern crate actix;
extern crate actix_web;
#[macro_use]
extern crate askama;

use actix::prelude::*;
use actix_web::{http, server, App, HttpRequest, HttpResponse, Result};
use askama::Template;

#[derive(Template)]
#[template(path = "user.html")]
struct UserTemplate<'a> {
    name: &'a str,
    text: &'a str,
}

#[derive(Template)]
#[template(path = "index.html")]
struct Index;

fn index(req: HttpRequest) -> Result<HttpResponse> {
    let s = if let Some(name) = req.query().get("name") {
        UserTemplate {
            name: name,
            text: "Welcome!",
        }.render()
            .unwrap()
    } else {
        Index.render().unwrap()
    };
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

fn main() {
    let sys = System::new("template-askama");

    // start http server
    server::new(move || App::new().resource("/", |r| r.method(http::Method::GET).f(index)))
        .bind("0.0.0.0:8080")
        .unwrap()
        .start();

    println!("Started http server: 127.0.0.1:8080");
    let _ = sys.run();
}
