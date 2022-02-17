use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, HttpRequest};
use actix_web::error::InternalError;
use actix_web::http::StatusCode;
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

async fn greet(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let name = req.match_info().get("name").unwrap_or("World");
    let body = Greet { name }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(body))
}

async fn page(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let id_string = req.match_info().get("id").unwrap().to_string();
    let id = &id_string.parse::<i32>().unwrap();
    let body = Page { id  }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(body))
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
            .route("/page-{id}", web::get().to(page))
            .route("/{name}", web::get().to(greet))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
