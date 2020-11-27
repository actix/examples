#[macro_use]
extern crate rbatis_macro_driver;

use actix_web::{get, middleware::Logger, App, HttpResponse, HttpServer, Responder};
use lazy_static::lazy_static;
use rbatis::{
    crud::{CRUDEnable, CRUD},
    rbatis::Rbatis,
};
use serde::{Deserialize, Serialize};

lazy_static! {
    pub static ref RB: Rbatis = Rbatis::new();
}

#[derive(CRUDEnable, Deserialize, Serialize, Clone, Debug)]
struct Person {
    name: String,
    username: String,
}

#[get("/person")]
async fn index() -> impl Responder {
    let person = Person {
        name: "Elisabeth .K".to_string(),
        username: "ellsabeth_".to_string(),
    };
    RB.save("", &person);

    let all_person: Vec<Person> = RB.list("").await.unwrap();
    HttpResponse::Ok().json(all_person)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    RB.link(&std::env::var("DB_URL").expect("DB_URL need to be set!!"))
        .await
        .unwrap();
    HttpServer::new(|| App::new().service(index))
        .bind("127.0.0.1:2121")?
        .run()
        .await
}
