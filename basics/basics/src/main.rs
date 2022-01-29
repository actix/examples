use actix_files as fs;
use actix_session::{CookieSession, Session};
use actix_web::http::{header, Method, StatusCode};
use actix_web::{
    error, get, middleware, web, App, Either, Error, HttpRequest, HttpResponse,
    HttpServer, Result,
};
use std::{env, io};
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;

/// favicon handler
#[get("/favicon")]
async fn favicon() -> Result<fs::NamedFile> {
    Ok(fs::NamedFile::open("static/favicon.ico")?)
}

/// simple index handler
#[get("/welcome")]
async fn welcome(session: Session, req: HttpRequest) -> Result<HttpResponse> {
    println!("{:?}", req);

    // session
    let mut counter = 1;
    if let Some(count) = session.get::<i32>("counter")? {
        println!("SESSION value: {}", count);
        counter = count + 1;
    }

    // set counter to session
    session.insert("counter", counter)?;

    // response
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../static/welcome.html")))
}

async fn default_handler(
    request: HttpRequest,
) -> Result<Either<fs::NamedFile, HttpResponse>> {
    match *request.method() {
        actix_web::http::Method::GET => {
            let file = fs::NamedFile::open("static/404.html")?
                .set_status_code(StatusCode::NOT_FOUND);
            Ok(Either::Left(file))
        }
        _ => Ok(Either::Right(HttpResponse::MethodNotAllowed().finish())),
    }
}

/// response body
async fn response_body(path: web::Path<String>) -> HttpResponse {
    let text = format!("Hello {}!", *path);

    let (tx, rx_body) = mpsc::unbounded_channel();
    let _ = tx.send(Ok::<_, Error>(web::Bytes::from(text)));

    HttpResponse::Ok().streaming(UnboundedReceiverStream::from(rx_body))
}

/// handler with path parameters like `/user/{name}/`
async fn with_param(req: HttpRequest, path: web::Path<(String,)>) -> HttpResponse {
    println!("{:?}", req);

    HttpResponse::Ok()
        .content_type("text/plain")
        .body(format!("Hello {}!", path.0))
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

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
            // async response body
            .service(
                web::resource("/async-body/{name}").route(web::get().to(response_body)),
            )
            .service(
                web::resource("/test").to(|req: HttpRequest| match *req.method() {
                    Method::GET => HttpResponse::Ok(),
                    Method::POST => HttpResponse::MethodNotAllowed(),
                    _ => HttpResponse::NotFound(),
                }),
            )
            .service(web::resource("/error").to(|| async {
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
                    .insert_header((header::LOCATION, "static/welcome.html"))
                    .finish()
            })))
            // default
            .default_service(web::route().to(default_handler))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
