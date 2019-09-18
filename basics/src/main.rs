#[macro_use]
extern crate actix_web;

use std::{env, io};

use actix_files as fs;
use actix_session::{CookieSession, Session};
use actix_web::http::{header, Method, StatusCode};
use actix_web::{
    error, guard, middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer,
    Result,
};
use bytes::Bytes;
use futures::unsync::mpsc;
use futures::{future::ok, Future, Stream};

/// favicon handler
#[get("/favicon")]
fn favicon() -> Result<fs::NamedFile> {
    Ok(fs::NamedFile::open("static/favicon.ico")?)
}

/// simple index handler
#[get("/welcome")]
fn welcome(session: Session, req: HttpRequest) -> Result<HttpResponse> {
    println!("{:?}", req);

    // session
    let mut counter = 1;
    if let Some(count) = session.get::<i32>("counter")? {
        println!("SESSION value: {}", count);
        counter = count + 1;
    }

    // set counter to session
    session.set("counter", counter)?;

    // response
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../static/welcome.html")))
}

/// 404 handler
fn p404() -> Result<fs::NamedFile> {
    Ok(fs::NamedFile::open("static/404.html")?.set_status_code(StatusCode::NOT_FOUND))
}

/// async handler
fn index_async(req: HttpRequest) -> impl Future<Item = HttpResponse, Error = Error> {
    println!("{:?}", req);

    ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(format!("Hello {}!", req.match_info().get("name").unwrap())))
}

/// async body
fn index_async_body(path: web::Path<String>) -> HttpResponse {
    let text = format!("Hello {}!", *path);

    let (tx, rx_body) = mpsc::unbounded();
    let _ = tx.unbounded_send(Bytes::from(text.as_bytes()));

    HttpResponse::Ok()
        .streaming(rx_body.map_err(|_| error::ErrorBadRequest("bad request")))
}

/// handler with path parameters like `/user/{name}/`
fn with_param(req: HttpRequest, path: web::Path<(String,)>) -> HttpResponse {
    println!("{:?}", req);

    HttpResponse::Ok()
        .content_type("text/plain")
        .body(format!("Hello {}!", path.0))
}

fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();
    let sys = actix_rt::System::new("basic-example");

    HttpServer::new(|| {
        App::new()
            // cookie session middleware
            .wrap(CookieSession::signed(&[0; 32]).secure(false))
            // enable logger - always register actix-web Logger middleware last
            .wrap(middleware::Logger::default())
            // register favicon
            .service(favicon)
            // register simple route, handle all methods
            .service(welcome)
            // with path parameters
            .service(web::resource("/user/{name}").route(web::get().to(with_param)))
            // async handler
            .service(
                web::resource("/async/{name}").route(web::get().to_async(index_async)),
            )
            // async handler
            .service(
                web::resource("/async-body/{name}")
                    .route(web::get().to(index_async_body)),
            )
            .service(
                web::resource("/test").to(|req: HttpRequest| match *req.method() {
                    Method::GET => HttpResponse::Ok(),
                    Method::POST => HttpResponse::MethodNotAllowed(),
                    _ => HttpResponse::NotFound(),
                }),
            )
            .service(web::resource("/error").to(|| {
                error::InternalError::new(
                    io::Error::new(io::ErrorKind::Other, "test"),
                    StatusCode::INTERNAL_SERVER_ERROR,
                )
            }))
            // static files
            .service(fs::Files::new("/static", "static").show_files_listing())
            // redirect
            .service(web::resource("/").route(web::get().to(|req: HttpRequest| {
                println!("{:?}", req);
                HttpResponse::Found()
                    .header(header::LOCATION, "static/welcome.html")
                    .finish()
            })))
            // default
            .default_service(
                // 404 for GET request
                web::resource("")
                    .route(web::get().to(p404))
                    // all requests that are not `GET`
                    .route(
                        web::route()
                            .guard(guard::Not(guard::Get()))
                            .to(HttpResponse::MethodNotAllowed),
                    ),
            )
    })
    .bind("127.0.0.1:8080")?
    .start();

    println!("Starting http server: 127.0.0.1:8080");
    sys.run()
}
