#[macro_use]
extern crate rbatis_macro_driver;
#[macro_use]
extern crate lazy_static;

use actix_web::{get,App,HttpServer,HttpResponse,Responder,middleware::Logger};
use serde::{Deserialize,Serialize};
use rbatis::{
    rbatis::Rbatis,
    crud::{
        CRUD,CRUDEnable,
    },
};

lazy_static! {
    pub static ref RB: Rbatis = Rbatis::new();
}

#[derive(CRUDEnable,Deserialize,Serialize,Clone,Debug)]
struct Person{
    name: String,
    username: String,
}

#[get("/person")]
async fn index() -> impl Responder{
    let person = Person{
        name: "Elisabeth .K".to_string(),
        username: "ellsabeth_".to_string(),
    };
    RB.save("",&person);

    let all_person: Vec<Person> = RB.list("").await.unwrap();
    HttpResponse::Ok().json(all_person)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    RB.link(&std::env::var("DB_URL").expect("DB_URL need to be set!!")).await.unwrap();
    HttpServer::new(||{
        App::new()
        .service(index)
    })
    .bind("127.0.0.1:2121")?
    .run()
    .await
}
