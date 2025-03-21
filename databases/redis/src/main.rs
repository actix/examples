use std::io;

use actix_web::{App, HttpResponse, HttpServer, Responder, error, middleware, web};
use serde::Deserialize;

async fn get_from_cache(redis: web::Data<redis::Client>) -> actix_web::Result<impl Responder> {
    let mut conn = redis
        .get_connection_manager()
        .await
        .map_err(error::ErrorInternalServerError)?;

    let res = redis::Cmd::mget(&["my_domain:one", "my_domain:two", "my_domain:three"])
        .query_async::<Vec<String>>(&mut conn)
        .await
        .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(res))
}

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
        .get_connection_manager()
        .await
        .map_err(error::ErrorInternalServerError)?;

    let res = redis::Cmd::mset(&[
        ("my_domain:one", info.one),
        ("my_domain:two", info.two),
        ("my_domain:three", info.three),
    ])
    .query_async::<String>(&mut conn)
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
        .get_connection_manager()
        .await
        .map_err(error::ErrorInternalServerError)?;

    let res = redis::Cmd::del(&["my_domain:one", "my_domain:two", "my_domain:three"])
        .query_async::<usize>(&mut conn)
        .await
        .map_err(error::ErrorInternalServerError)?;

    // not strictly necessary, but successful DEL operations return the number of keys deleted
    if res == 3 {
        Ok(HttpResponse::Ok().body("successfully deleted values"))
    } else {
        tracing::error!("deleted {res} keys");
        Ok(HttpResponse::InternalServerError().finish())
    }
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    tracing::info!("starting HTTP server at http://localhost:8080");

    let redis = redis::Client::open("redis://127.0.0.1:6379").unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(redis.clone()))
            .service(
                web::resource("/stuff")
                    .route(web::get().to(get_from_cache))
                    .route(web::post().to(cache_stuff))
                    .route(web::delete().to(del_stuff)),
            )
            .wrap(middleware::NormalizePath::trim())
            .wrap(middleware::Logger::default())
    })
    .workers(2)
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
