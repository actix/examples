use std::io;

use actix_web::{error, middleware, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CacheInfo {
    one: String,
    two: String,
    three: String,
}

async fn cache_stuff(
    web::Json(info): web::Json<CacheInfo>,
    redis: web::Data<redis::Client>,
) -> actix_web::Result<impl Responder> {
    let mut conn = redis
        .get_tokio_connection_manager()
        .await
        .map_err(error::ErrorInternalServerError)?;

    let res = redis::Cmd::set_multiple(&[
        ("my_domain:one", info.one),
        ("my_domain:two", info.two),
        ("my_domain:three", info.three),
    ])
    .query_async::<_, String>(&mut conn)
    .await
    .map_err(error::ErrorInternalServerError)?;

    // not strictly necessary, but successful SET operations return "OK"
    if res == "OK" {
        Ok(HttpResponse::Ok().body("successfully cached values"))
    } else {
        Ok(HttpResponse::InternalServerError().finish())
    }
}

async fn del_stuff(redis: web::Data<redis::Client>) -> actix_web::Result<impl Responder> {
    let mut conn = redis
        .get_tokio_connection_manager()
        .await
        .map_err(error::ErrorInternalServerError)?;

    let res = redis::Cmd::del(&["my_domain:one", "my_domain:two", "my_domain:three"])
        .query_async::<_, usize>(&mut conn)
        .await
        .map_err(error::ErrorInternalServerError)?;

    // not strictly necessary, but successful DEL operations return the number of keys deleted
    if res == 3 {
        Ok(HttpResponse::Ok().body("successfully deleted values"))
    } else {
        log::error!("deleted {res} keys");
        Ok(HttpResponse::InternalServerError().finish())
    }
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("starting HTTP server at http://localhost:8080");

    let redis = redis::Client::open("redis://127.0.0.1:6379").unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(redis.clone()))
            .wrap(middleware::Logger::default())
            .service(
                web::resource("/stuff")
                    .route(web::post().to(cache_stuff))
                    .route(web::delete().to(del_stuff)),
            )
    })
    .workers(2)
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
