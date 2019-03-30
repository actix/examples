#[macro_use]
extern crate prost_derive;

use actix_web::{middleware, web, App, Error, HttpResponse, HttpServer};
use futures::Future;
mod protobuf;
use protobuf::ProtoBufResponseBuilder;

#[derive(Clone, Debug, PartialEq, Message)]
pub struct MyObj {
    #[prost(int32, tag = "1")]
    pub number: i32,
    #[prost(string, tag = "2")]
    pub name: String,
}

/// This handler uses `ProtoBufMessage` for loading protobuf object.
fn index(pl: web::Payload) -> impl Future<Item = HttpResponse, Error = Error> {
    protobuf::ProtoBufMessage::new(pl)
        .from_err() // convert all errors into `Error`
        .and_then(|val: MyObj| {
            println!("model: {:?}", val);
            Ok(HttpResponse::Ok().protobuf(val)?) // <- send response
        })
}

fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info,actix_server=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(web::resource("/").route(web::post().to_async(index)))
    })
    .bind("127.0.0.1:8080")?
    .run()
}
