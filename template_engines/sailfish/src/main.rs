use actix_web::{
    error, get,
    middleware::{Compress, Logger},
    web, App, HttpServer, Responder,
};
use actix_web_lab::respond::Html;
use sailfish::TemplateOnce;

#[derive(TemplateOnce)]
#[template(path = "actix.stpl")]
struct Greet<'a> {
    name: &'a str,
}

#[derive(TemplateOnce)]
#[template(path = "page.stpl")]
struct Page<'a> {
    id: &'a i32,
}

#[get("/{name}")]
async fn greet(params: web::Path<(String,)>) -> actix_web::Result<impl Responder> {
    let body = Greet { name: &params.0 }
        .render_once()
        .map_err(error::ErrorInternalServerError)?;

    Ok(Html(body))
}

#[get("/page-{id:\\d+}")]
async fn page(params: web::Path<(i32,)>) -> actix_web::Result<impl Responder> {
    let body = Page { id: &params.0 }
        .render_once()
        .map_err(error::ErrorInternalServerError)?;

    Ok(Html(body))
}

#[get("/")]
async fn hello() -> impl Responder {
    Html("<p>Hello world!</p>".to_string())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("starting HTTP server at http://localhost:8080");

    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(page)
            .service(greet)
            .wrap(Compress::default())
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", 8080))?
    .workers(1)
    .run()
    .await
}
