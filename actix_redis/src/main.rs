extern crate actix;
extern crate actix_redis;
extern crate actix_web;
extern crate env_logger;
extern crate futures;
#[macro_use]
extern crate redis_async;
extern crate serde;
#[macro_use]
extern crate serde_derive;

use actix::prelude::*;
use actix_redis::{Command, Error as ARError, RedisActor};
use actix_web::{
    http::Method, middleware, server, App, AsyncResponder, Error as AWError,
    HttpRequest, HttpResponse, Json,
};
use futures::future::{join_all, Future};
use redis_async::resp::RespValue;
use std::sync::Arc;

#[derive(Deserialize)]
pub struct CacheInfo {
    one: String,
    two: String,
    three: String,
}

fn cache_stuff(
    (info, req): (Json<CacheInfo>, HttpRequest<AppState>),
) -> impl Future<Item = HttpResponse, Error = AWError> {
    let info = info.into_inner();
    let redis = req.state().redis_addr.clone();

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
    let info_set = join_all(vec![one, two, three].into_iter());

    info_set
    .map_err(AWError::from)
    .and_then(|res: Vec<Result<RespValue, ARError>>|
        // successful operations return "OK", so confirm that all returned as so
        if !res.iter().all(|res| match res {
                Ok(RespValue::SimpleString(x)) if x=="OK" => true,
                                                        _ => false
            }) {
            Ok(HttpResponse::InternalServerError().finish())
        } else {
            Ok(HttpResponse::Ok().body("successfully cached values"))
        }
    )
    .responder()
}

fn del_stuff(
    req: HttpRequest<AppState>,
) -> impl Future<Item = HttpResponse, Error = AWError> {
    let redis = req.state().redis_addr.clone();

    redis
        .send(Command(resp_array![
            "DEL",
            "mydomain:one",
            "mydomain:two",
            "mydomain:three"
        ]))
        .map_err(AWError::from)
        .and_then(|res: Result<RespValue, ARError>| match &res {
            Ok(RespValue::Integer(x)) if x == &3 => {
                Ok(HttpResponse::Ok().body("successfully deleted values"))
            }
            _ => {
                println!("---->{:?}", res);
                Ok(HttpResponse::InternalServerError().finish())
            }
        })
        .responder()
}

pub struct AppState {
    pub redis_addr: Arc<Addr<RedisActor>>,
}

fn main() {
    ::std::env::set_var("RUST_LOG", "actix_web=info,actix_redis=info");
    env_logger::init();
    let sys = actix::System::new("actix_redis_ex");

    server::new(|| {
        let redis_addr = Arc::new(RedisActor::start("127.0.0.1:6379"));
        let app_state = AppState { redis_addr };

        App::with_state(app_state)
            .middleware(middleware::Logger::default())
            .resource("/stuff", |r| {
                r.method(Method::POST).with_async(cache_stuff);
                r.method(Method::DELETE).with_async(del_stuff)
            })
    })
    .bind("0.0.0.0:8080")
    .unwrap()
    .workers(1)
    .start();

    let _ = sys.run();
}
