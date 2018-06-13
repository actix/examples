#![allow(unused_variables)]
#![cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value))]

extern crate actix;
extern crate actix_web;
extern crate bytes;
extern crate env_logger;
extern crate futures;

use bytes::Bytes;
use futures::sync::mpsc;
use futures::Stream;

use actix_web::http::{header, Method, StatusCode};
use actix_web::middleware::session::{self, RequestSession};
use actix_web::{
    error, fs, middleware, pred, server, App, Error, HttpRequest, HttpResponse, Path,
    Result,
};
use futures::future::{result, FutureResult};
use std::{env, io};

/// favicon handler
fn favicon(req: HttpRequest) -> Result<fs::NamedFile> {
    Ok(fs::NamedFile::open("static/favicon.ico")?)
}

/// simple index handler
fn welcome(mut req: HttpRequest) -> Result<HttpResponse> {
    println!("{:?}", req);

    // example of ...
    if let Ok(ch) = req.poll() {
        if let futures::Async::Ready(Some(d)) = ch {
            println!("{}", String::from_utf8_lossy(d.as_ref()));
        }
    }

    // session
    let mut counter = 1;
    if let Some(count) = req.session().get::<i32>("counter")? {
        println!("SESSION value: {}", count);
        counter = count + 1;
        req.session().set("counter", counter)?;
    } else {
        req.session().set("counter", counter)?;
    }

    // response
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../static/welcome.html")))
}

/// 404 handler
fn p404(req: HttpRequest) -> Result<fs::NamedFile> {
    Ok(fs::NamedFile::open("static/404.html")?.set_status_code(StatusCode::NOT_FOUND))
}

/// async handler
fn index_async(req: HttpRequest) -> FutureResult<HttpResponse, Error> {
    println!("{:?}", req);

    result(Ok(HttpResponse::Ok().content_type("text/html").body(
        format!("Hello {}!", req.match_info().get("name").unwrap()),
    )))
}

/// async body
fn index_async_body(path: Path<String>) -> HttpResponse {
    let text = format!("Hello {}!", *path);

    let (tx, rx_body) = mpsc::unbounded();
    let _ = tx.unbounded_send(Bytes::from(text.as_bytes()));

    HttpResponse::Ok()
        .streaming(rx_body.map_err(|e| error::ErrorBadRequest("bad request")))
}

/// handler with path parameters like `/user/{name}/`
fn with_param(req: HttpRequest) -> HttpResponse {
    println!("{:?}", req);

    HttpResponse::Ok()
        .content_type("test/plain")
        .body(format!("Hello {}!", req.match_info().get("name").unwrap()))
}

fn main() {
    env::set_var("RUST_LOG", "actix_web=debug");
    env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();
    let sys = actix::System::new("basic-example");

    let addr = server::new(
        || App::new()
            // enable logger
            .middleware(middleware::Logger::default())
            // cookie session middleware
            .middleware(session::SessionStorage::new(
                session::CookieSessionBackend::signed(&[0; 32]).secure(false)
            ))
            // register favicon
            .resource("/favicon", |r| r.f(favicon))
            // register simple route, handle all methods
            .resource("/welcome", |r| r.f(welcome))
            // with path parameters
            .resource("/user/{name}", |r| r.method(Method::GET).f(with_param))
            // async handler
            .resource("/async/{name}", |r| r.method(Method::GET).a(index_async))
            // async handler
            .resource("/async-body/{name}", |r| r.method(Method::GET).with(index_async_body))
            .resource("/test", |r| r.f(|req| {
                match *req.method() {
                    Method::GET => HttpResponse::Ok(),
                    Method::POST => HttpResponse::MethodNotAllowed(),
                    _ => HttpResponse::NotFound(),
                }
            }))
            .resource("/error", |r| r.f(|req| {
                error::InternalError::new(
                    io::Error::new(io::ErrorKind::Other, "test"), StatusCode::INTERNAL_SERVER_ERROR)
            }))
            // static files
            .handler("/static", fs::StaticFiles::new("static"))
            // redirect
            .resource("/", |r| r.method(Method::GET).f(|req| {
                println!("{:?}", req);
                HttpResponse::Found()
                    .header(header::LOCATION, "static/index.html")
                    .finish()
            }))
            // default
            .default_resource(|r| {
                // 404 for GET request
                r.method(Method::GET).f(p404);

                // all requests that are not `GET`
                r.route().filter(pred::Not(pred::Get())).f(
                    |req| HttpResponse::MethodNotAllowed());
            }))

        .bind("127.0.0.1:8080").expect("Can not bind to 127.0.0.1:8080")
        .shutdown_timeout(0)    // <- Set shutdown timeout to 0 seconds (default 60s)
        .start();

    println!("Starting http server: 127.0.0.1:8080");
    let _ = sys.run();
}
