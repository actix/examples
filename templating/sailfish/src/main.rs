use actix_web::{
    App, HttpServer, Responder, error, get,
    middleware::{Compress, Logger},
    web,
};
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

    Ok(web::Html::new(body))
}

#[get("/page-{id:\\d+}")]
async fn page(params: web::Path<(i32,)>) -> actix_web::Result<impl Responder> {
    let body = Page { id: &params.0 }
        .render_once()
        .map_err(error::ErrorInternalServerError)?;

    Ok(web::Html::new(body))
}

#[get("/")]
async fn hello() -> impl Responder {
    web::Html::new("<p>Hello world!</p>".to_owned())
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
