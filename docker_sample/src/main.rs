#[macro_use]
extern crate actix_web;

use actix_web::{App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn index() -> impl Responder {
    println!("GET: /");
    HttpResponse::Ok().body("Hello world!")
}

#[get("/again")]
async fn again() -> impl Responder {
    println!("GET: /again");
    HttpResponse::Ok().body("Hello world again!")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    println!("Starting actix-web server");

    HttpServer::new(|| App::new().service(index).service(again))
        .bind("0.0.0.0:5000")?
        .run()
        .await
}
