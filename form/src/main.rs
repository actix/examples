#[macro_use]
extern crate serde_derive;

use actix_web::{
    middleware, web, App, HttpRequest, HttpResponse, HttpServer, Responder, Result,
};

struct AppState {
    foo: String,
}

fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .data(AppState {
                foo: "bar".to_string(),
            })
            .wrap(middleware::Logger::default())
            .service(web::resource("/").route(web::get().to(index)))
            .service(web::resource("/post1").route(web::post().to(handle_post_1)))
            .service(web::resource("/post2").route(web::post().to(handle_post_2)))
            .service(web::resource("/post3").route(web::post().to(handle_post_3)))
    })
    .bind("127.0.0.1:8080")?
    .run()
}

fn index() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../static/form.html")))
}

#[derive(Deserialize)]
pub struct MyParams {
    name: String,
}

/// Simple handle POST request
fn handle_post_1(params: web::Form<MyParams>) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/plain")
        .body(format!("Your name is {}", params.name)))
}

/// State and POST Params
fn handle_post_2(
    state: web::Data<AppState>,
    params: web::Form<MyParams>,
) -> HttpResponse {
    HttpResponse::Ok().content_type("text/plain").body(format!(
        "Your name is {}, and in AppState I have foo: {}",
        params.name, state.foo
    ))
}

/// Request and POST Params
fn handle_post_3(req: HttpRequest, params: web::Form<MyParams>) -> impl Responder {
    println!("Handling POST request: {:?}", req);

    HttpResponse::Ok()
        .content_type("text/plain")
        .body(format!("Your name is {}", params.name))
}
