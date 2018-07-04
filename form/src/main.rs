extern crate actix;
extern crate actix_web;

#[macro_use]
extern crate serde_derive;

use actix_web::{
    http, middleware, server, App, Form, HttpRequest, HttpResponse, Result, State,
};

struct AppState {
    foo: String,
}

fn main() {
    let sys = actix::System::new("form-example");

    let _addr = server::new(|| {
        App::with_state(AppState {
            foo: "bar".to_string(),
        }).middleware(middleware::Logger::default())
            .resource("/", |r| {
                r.method(http::Method::GET).with(index);
            })
            .resource("/post1", |r| {
                r.method(http::Method::POST).with(handle_post_1)
            })
            .resource("/post2", |r| {
                r.method(http::Method::POST).with(handle_post_2)
            })
            .resource("/post3", |r| {
                r.method(http::Method::POST).with(handle_post_3)
            })
    }).bind("127.0.0.1:8080")
        .expect("Can not bind to 127.0.0.1:8080")
        .start();

    println!("Starting http server: 127.0.0.1:8080");
    let _ = sys.run();
}

fn index(_req: HttpRequest<AppState>) -> Result<HttpResponse> {
    Ok(HttpResponse::build(http::StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../static/form.html")))
}

#[derive(Deserialize)]
pub struct MyParams {
    name: String,
}

/// Simple handle POST request
fn handle_post_1(params: Form<MyParams>) -> Result<HttpResponse> {
    Ok(HttpResponse::build(http::StatusCode::OK)
        .content_type("text/plain")
        .body(format!("Your name is {}", params.name)))
}

/// State and POST Params
fn handle_post_2(
    (state, params): (State<AppState>, Form<MyParams>),
) -> Result<HttpResponse> {
    Ok(HttpResponse::build(http::StatusCode::OK)
        .content_type("text/plain")
        .body(format!(
            "Your name is {}, and in AppState I have foo: {}",
            params.name, state.foo
        )))
}

/// Request and POST Params
fn handle_post_3(
    (req, params): (HttpRequest<AppState>, Form<MyParams>),
) -> Result<HttpResponse> {
    println!("Handling POST request: {:?}", req);
    Ok(HttpResponse::build(http::StatusCode::OK)
        .content_type("text/plain")
        .body(format!("Your name is {}", params.name)))
}
