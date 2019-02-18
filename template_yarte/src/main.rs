extern crate actix;
extern crate actix_web;

use std::collections::HashMap;

use actix_web::{http, server, App, HttpResponse, Query, Result};
use yarte::Template;

#[derive(Template)]
#[template(path = "user.html")]
struct UserTemplate<'a> {
    name: &'a str,
    text: &'a str,
}

#[derive(Template)]
#[template(path = "index.html")]
struct Index;

fn index(query: Query<HashMap<String, String>>) -> Result<HttpResponse> {
    let s = if let Some(name) = query.get("name") {
        UserTemplate {
            name,
            text: "Welcome!",
        }
        .to_string()
    } else {
        Index.to_string()
    };
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

fn main() {
    let sys = actix::System::new("template-yarte");

    // start http server
    server::new(move || {
        App::new().resource("/", |r| r.method(http::Method::GET).with(index))
    })
    .bind("127.0.0.1:8080")
    .unwrap()
    .start();

    println!("Started http server: 127.0.0.1:8080");
    let _ = sys.run();
}
