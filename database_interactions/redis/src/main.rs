use actix::prelude::*;
use actix_redis::{Command, RedisActor};
use actix_web::{middleware, web, App, Error as AWError, HttpResponse, HttpServer};
use futures::future::join_all;
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
) -> Result<HttpResponse, AWError> {
    let info = info.into_inner();

    let one = redis.send(Command(resp_array!["SET", "mydomain:one", info.one]));
    let two = redis.send(Command(resp_array!["SET", "mydomain:two", info.two]));
    let three = redis.send(Command(resp_array!["SET", "mydomain:three", info.three]));

    // Creates a future which represents a collection of the results of the futures
    // given. The returned future will drive execution for all of its underlying futures,
    // collecting the results into a destination `Vec<RespValue>` in the same order as they
    // were provided. If any future returns an error then all other futures will be
    // canceled and an error will be returned immediately. If all futures complete
    // successfully, however, then the returned future will succeed with a `Vec` of
    // all the successful results.
    let res: Vec<Result<RespValue, AWError>> =
        join_all(vec![one, two, three].into_iter())
            .await
            .into_iter()
            .map(|item| {
                item.map_err(AWError::from)
                    .and_then(|res| res.map_err(AWError::from))
            })
            .collect();

    // successful operations return "OK", so confirm that all returned as so
    if !res.iter().all(|res| match res {
        Ok(RespValue::SimpleString(x)) if x == "OK" => true,
        _ => false,
    }) {
        Ok(HttpResponse::InternalServerError().finish())
    } else {
        Ok(HttpResponse::Ok().body("successfully cached values"))
    }
}

async fn del_stuff(redis: web::Data<Addr<RedisActor>>) -> Result<HttpResponse, AWError> {
    let res = redis
        .send(Command(resp_array![
            "DEL",
            "mydomain:one",
            "mydomain:two",
            "mydomain:three"
        ]))
        .await?;

    match res {
        Ok(RespValue::Integer(x)) if x == 3 => {
            Ok(HttpResponse::Ok().body("successfully deleted values"))
        }
        _ => {
            println!("---->{:?}", res);
            Ok(HttpResponse::InternalServerError().finish())
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=trace,actix_redis=trace");
    env_logger::init();

    HttpServer::new(|| {
        let redis_addr = RedisActor::start("127.0.0.1:6379");

        App::new()
            .data(redis_addr)
            .wrap(middleware::Logger::default())
            .service(
                web::resource("/stuff")
                    .route(web::post().to(cache_stuff))
                    .route(web::delete().to(del_stuff)),
            )
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
