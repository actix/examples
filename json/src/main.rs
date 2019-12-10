use actix_web::{middleware, web, App, Error, HttpResponse, HttpRequest, HttpServer};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct MyUser {
    name: String,
}

async fn echo(
    item: web::Json<MyUser>,
    req: HttpRequest,
) -> Result<HttpResponse, Error> {
    println!("{:?}", req);
    Ok(HttpResponse::Ok().json(item.0))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .service(web::resource("/")
                    .data(
                        web::JsonConfig::default()
                    )
                    .route(web::post().to(echo)),
            )
    })
    .bind("0.0.0.0:3000")?
    .start()
    .await
}
