#[macro_use]
extern crate redis_async;
#[macro_use]
extern crate serde_derive;

use std::io;

use actix::prelude::*;
use actix_redis::{Command, Error as ARError, RedisActor};
use actix_web::{
    error::ErrorInternalServerError, middleware, web, App, Error as AWError,
    HttpResponse, HttpServer,
};
use futures::future::{join_all, Future};
use redis_async::resp::RespValue;

#[derive(Deserialize)]
pub struct CacheInfo {
    one: String,
    two: String,
    three: String,
}

fn cache_stuff(
    info: web::Json<CacheInfo>,
    state: web::Data<AppState>,
) -> impl Future<Item = HttpResponse, Error = AWError> {
    let info = info.into_inner();

    let one = state
        .redis
        .send(Command(resp_array!["SET", "mydomain:one", info.one]));
    let two = state
        .redis
        .send(Command(resp_array!["SET", "mydomain:two", info.two]));
    let three =
        state
            .redis
            .send(Command(resp_array!["SET", "mydomain:three", info.three]));

    // Creates a future which represents a collection of the results of the futures
    // given. The returned future will drive execution for all of its underlying futures,
    // collecting the results into a destination `Vec<RespValue>` in the same order as they
    // were provided. If any future returns an error then all other futures will be
    // canceled and an error will be returned immediately. If all futures complete
    // successfully, however, then the returned future will succeed with a `Vec` of
    // all the successful results.
    let info_set = join_all(vec![one, two, three].into_iter());

    info_set.map_err(|e| ErrorInternalServerError(e)).and_then(
        |res: Vec<Result<RespValue, ARError>>|
            // successful operations return "OK", so confirm that all returned as so
            if !res.iter().all(|res| match res {
                Ok(RespValue::SimpleString(x)) if x == "OK" => true,
                _ => false
            }) {
                Ok(HttpResponse::InternalServerError().finish())
            } else {
                Ok(HttpResponse::Ok().body("successfully cached values"))
            },
    )
}

fn del_stuff(
    state: web::Data<AppState>,
) -> impl Future<Item = HttpResponse, Error = AWError> {
    state
        .redis
        .send(Command(resp_array![
            "DEL",
            "mydomain:one",
            "mydomain:two",
            "mydomain:three"
        ]))
        .map_err(|e| ErrorInternalServerError(e))
        .and_then(|res: Result<RespValue, ARError>| match &res {
            Ok(RespValue::Integer(x)) if x == &3 => {
                Ok(HttpResponse::Ok().body("successfully deleted values"))
            }
            _ => {
                println!("---->{:?}", res);
                Ok(HttpResponse::InternalServerError().finish())
            }
        })
}

#[derive(Clone)]
pub struct AppState {
    pub redis: Addr<RedisActor>,
}

pub fn main() -> Result<(), io::Error> {
    ::std::env::set_var("RUST_LOG", "actix_web=info,actix_redis=info");
    env_logger::init();
    let sys = actix_rt::System::new("actix_redis_ex");

    HttpServer::new(|| {
        App::new()
            .data(AppState {
                redis: RedisActor::start("127.0.0.1:6379"),
            })
            .wrap(middleware::Logger::default())
            .service(
                web::resource("/stuff")
                    .route(web::post().to_async(cache_stuff))
                    .route(web::delete().to_async(del_stuff)),
            )
    })
    .bind("0.0.0.0:8080")?
    .start();

    sys.run()
}
