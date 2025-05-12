use std::{convert::Infallible, io};

use actix_files::{Files, NamedFile};
use actix_session::{Session, SessionMiddleware, storage::CookieSessionStore};
use actix_web::{
    App, Either, HttpRequest, HttpResponse, HttpServer, Responder, Result, error, get,
    http::{
        Method, StatusCode,
        header::{self, ContentType},
    },
    middleware, web,
};
use actix_web_lab::extract::Path;
use async_stream::stream;

// NOTE: Not a suitable session key for production.
static SESSION_SIGNING_KEY: &[u8] = &[0; 64];

/// favicon handler
#[get("/favicon")]
async fn favicon() -> Result<impl Responder> {
    Ok(NamedFile::open("static/favicon.ico")?)
}

/// simple index handler
#[get("/welcome")]
async fn welcome(req: HttpRequest, session: Session) -> Result<HttpResponse> {
    println!("{req:?}");

    // session
    let mut counter = 1;
    if let Some(count) = session.get::<i32>("counter")? {
        println!("SESSION value: {count}");
        counter = count + 1;
    }

    // set counter to session
    session.insert("counter", counter)?;

    // response
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type(ContentType::plaintext())
        .body(include_str!("../static/welcome.html")))
}

async fn default_handler(req_method: Method) -> Result<impl Responder> {
    match req_method {
        Method::GET => {
            let file = NamedFile::open("static/404.html")?
                .customize()
                .with_status(StatusCode::NOT_FOUND);
            Ok(Either::Left(file))
        }
        _ => Ok(Either::Right(HttpResponse::MethodNotAllowed().finish())),
    }
}

async fn streaming_response(path: web::Path<String>) -> HttpResponse {
    let name = path.into_inner();

    HttpResponse::Ok()
        .content_type(ContentType::plaintext())
        .streaming(stream! {
            yield Ok::<_, Infallible>(web::Bytes::from("Hello "));
            yield Ok::<_, Infallible>(web::Bytes::from(name));
            yield Ok::<_, Infallible>(web::Bytes::from("!"));
        })
}

/// handler with path parameters like `/user/{name}`
async fn with_param(req: HttpRequest, Path((name,)): Path<(String,)>) -> HttpResponse {
    println!("{req:?}");

    HttpResponse::Ok()
        .content_type(ContentType::plaintext())
        .body(format!("Hello {name}!"))
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    examples_common::init_standard_logger();

    // random key means that restarting server will invalidate existing session cookies
    let key = actix_web::cookie::Key::from(SESSION_SIGNING_KEY);

    tracing::info!("Starting HTTP server at http://localhost:8080");

    HttpServer::new(move || {
        App::new()
            // enable automatic response compression - usually register this first
            .wrap(middleware::Compress::default())
            // cookie session middleware
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), key.clone())
                    .cookie_secure(false)
                    .build(),
            )
            // enable logger - always register Actix Web Logger middleware last
            .wrap(middleware::Logger::default().log_target("@"))
            // register favicon
            .service(favicon)
            // register simple route, handle all methods
            .service(welcome)
            // with path parameters
            .service(web::resource("/user/{name}").route(web::get().to(with_param)))
            // async response body
            .service(web::resource("/async-body/{name}").route(web::get().to(streaming_response)))
            .service(
                web::resource("/test").to(|req: HttpRequest| match *req.method() {
                    Method::GET => HttpResponse::Ok(),
                    Method::POST => HttpResponse::MethodNotAllowed(),
                    _ => HttpResponse::NotFound(),
                }),
            )
            .service(web::resource("/error").to(|| async {
                error::InternalError::new(
                    io::Error::other("test"),
                    StatusCode::INTERNAL_SERVER_ERROR,
                )
            }))
            // static files
            .service(Files::new("/static", "static").show_files_listing())
            // redirect
            .service(
                web::resource("/").route(web::get().to(|req: HttpRequest| async move {
                    println!("{req:?}");
                    HttpResponse::Found()
                        .insert_header((header::LOCATION, "static/welcome.html"))
                        .finish()
                })),
            )
            // default
            .default_service(web::to(default_handler))
    })
    .bind(("127.0.0.1", 8080))?
    .workers(2)
    .run()
    .await
}
