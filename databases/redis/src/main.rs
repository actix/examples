use actix::prelude::*;
use actix_redis::{Command, RedisActor};
use actix_web::{error, middleware, web, App, HttpResponse, HttpServer};
use futures_util::future::try_join_all;
use redis_async::{resp::RespValue, resp_array};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CacheInfo {
    one: String,
    two: String,
    three: String,
}

async fn cache_stuff(
    info: web::Json<CacheInfo>,
    redis: web::Data<Addr<RedisActor>>,
) -> actix_web::Result<HttpResponse> {
    let info = info.into_inner();

    let one = redis.send(Command(resp_array!["SET", "mydomain:one", info.one]));
    let two = redis.send(Command(resp_array!["SET", "mydomain:two", info.two]));
    let three = redis.send(Command(resp_array!["SET", "mydomain:three", info.three]));

    // Asynchronously collects the results of the futures given. The returned future will drive
    // execution for all of its underlying futures, collecting the results into a destination
    // `Vec<RespValue>` in the same order as they were provided. If any future returns an error then
    // all other futures will be canceled and an error will be returned immediately. If all futures
    // complete successfully, however, then the returned future will succeed with a `Vec` of all the
    // successful results.
    let res = try_join_all([one, two, three])
        .await
        .map_err(error::ErrorInternalServerError)?
        .into_iter()
        .map(|item| item.map_err(error::ErrorInternalServerError))
        .collect::<Result<Vec<_>, _>>()?;

    // successful operations return "OK", so confirm that all returned as so
    if res
        .iter()
        .all(|res| matches!(res, RespValue::SimpleString(x) if x == "OK"))
    {
        Ok(HttpResponse::Ok().body("successfully cached values"))
    } else {
        Ok(HttpResponse::InternalServerError().finish())
    }
}

async fn del_stuff(redis: web::Data<Addr<RedisActor>>) -> actix_web::Result<HttpResponse> {
    let res = redis
        .send(Command(resp_array![
            "DEL",
            "mydomain:one",
            "mydomain:two",
            "mydomain:three"
        ]))
        .await
        .map_err(error::ErrorInternalServerError)?
        .map_err(error::ErrorInternalServerError)?;

    match res {
        RespValue::Integer(x) if x == 3 => {
            Ok(HttpResponse::Ok().body("successfully deleted values"))
        }

        _ => {
            log::error!("{res:?}");
            Ok(HttpResponse::InternalServerError().finish())
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("starting HTTP server at http://localhost:8080");

    HttpServer::new(|| {
        let redis_addr = RedisActor::start("127.0.0.1:6379");

        App::new()
            .app_data(web::Data::new(redis_addr))
            .wrap(middleware::Logger::default())
            .service(
                web::resource("/stuff")
                    .route(web::post().to(cache_stuff))
                    .route(web::delete().to(del_stuff)),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
