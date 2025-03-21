use actix_protobuf::{ProtoBuf, ProtoBufResponseBuilder as _};
use actix_web::{App, HttpResponse, HttpServer, Result, middleware, web};
use prost::Message;

#[derive(Clone, PartialEq, Eq, Message)]
pub struct MyObj {
    #[prost(int32, tag = "1")]
    pub number: i32,

    #[prost(string, tag = "2")]
    pub name: String,
}

async fn index(msg: ProtoBuf<MyObj>) -> Result<HttpResponse> {
    log::info!("model: {msg:?}");
    HttpResponse::Ok().protobuf(msg.0) // <- send response
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("starting HTTP server at http://localhost:8080");

    HttpServer::new(|| {
        App::new()
            .service(web::resource("/").route(web::post().to(index)))
            .wrap(middleware::Logger::default())
    })
    .workers(1)
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
