use actix_web::{App, HttpResponse, HttpServer, web};

mod simple;

// You can move this struct to a separate file.
// this struct below just for example.
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct HttpData {
    pub data: String,
}
// this implementation is optional
impl Default for HttpData {
    fn default() -> Self {
        Self {
            data: "Hello this is success response!".to_string(),
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("starting HTTP server at http://localhost:8080");

    HttpServer::new(|| {
        App::new().wrap(simple::ReturnHttpResponse).service(
            web::resource("/").to(|| async { HttpResponse::Ok().json(HttpData::default()) }),
        )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
