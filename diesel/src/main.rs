//! Actix web diesel example
//!
//! Diesel does not support tokio, so we have to run it in separate threads.
//! Actix supports sync actors by default, so we going to create sync actor
//! that use diesel. Technically sync actors are worker style actors, multiple
//! of them can run in parallel and process messages from same queue.
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate diesel;
extern crate actix;
extern crate actix_web;
extern crate env_logger;
extern crate futures;
extern crate r2d2;
extern crate uuid;
extern crate bytes;


use bytes::BytesMut;
use actix::prelude::*;
use actix_web::{
    http, middleware, server, App, AsyncResponder, FutureResponse, HttpResponse, Path, Error, HttpRequest,
    State, HttpMessage, error
};

use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use futures::{Future, Stream};

mod db;
mod models;
mod schema;

use db::{CreateUser, DbExecutor};

/// State with DbExecutor address
struct AppState {
    db: Addr<DbExecutor>,
}

/// Async request handler
fn add(
    (name, state): (Path<String>, State<AppState>),
) -> FutureResponse<HttpResponse> {
    // send async `CreateUser` message to a `DbExecutor`
    state
        .db
        .send(CreateUser {
            name: name.into_inner(),
        })
        .from_err()
        .and_then(|res| match res {
            Ok(user) => Ok(HttpResponse::Ok().json(user)),
            Err(_) => Ok(HttpResponse::InternalServerError().into()),
        })
        .responder()
}

#[derive(Debug, Serialize, Deserialize)]
struct MyUser {
    name: String
}

const MAX_SIZE: usize = 262_144; // max payload size is 256k

/// This handler manually load request payload and parse json object
fn index_add(req: &HttpRequest<AppState>) -> Box<Future<Item = HttpResponse, Error = Error>> {
    // HttpRequest::payload() is stream of Bytes objects
    req.payload()
        // `Future::from_err` acts like `?` in that it coerces the error type from
        // the future into the final error type
        .from_err()

        // `fold` will asynchronously read each chunk of the request body and
        // call supplied closure, then it resolves to result of closure
        .fold(BytesMut::new(), move |mut body, chunk| {
            // limit max size of in-memory payload
            if (body.len() + chunk.len()) > MAX_SIZE {
                Err(error::ErrorBadRequest("overflow"))
            } else {
                body.extend_from_slice(&chunk);
                Ok(body)
            }
        })
        // `Future::and_then` can be used to merge an asynchronous workflow with a
        // synchronous workflow
        .and_then(|body| {
            // body is loaded, now we can deserialize serde-json
            let r_obj = serde_json::from_slice::<MyUser>(&body);

            // Send to the db for create
            match r_obj {
                Ok(obj) => {
                req.state()
                    .db
                    .send(CreateUser {
                        name: obj.name,
                    })
                    .from_err()
                    .and_then(|res| match res {
                        Ok(user) => Ok(HttpResponse::Ok().json(user)),
                        Err(_) => Ok(HttpResponse::InternalServerError().into()),
                    })
                }
                Err(_) => {
                    Err(error::ErrorBadRequest("Json Decode Failed"))
                }
            }
        })
        .responder()
}

fn extract_item_limit((item, req): (Json<MyUser>, HttpRequest<AppState>)) -> HttpResponse {
    req.state()
        .db
        .send(CreateUser {
            name: item.name,
        })
        .from_err()
        .and_then(|res| match res {
            Ok(user) => HttpResponse::Ok().json(user),
            Err(_) => HttpResponse::InternalServerError().into(),
        })
}

fn main() {
    ::std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    let sys = actix::System::new("diesel-example");

    // Start 3 db executor actors
    let manager = ConnectionManager::<SqliteConnection>::new("test.db");
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    let addr = SyncArbiter::start(3, move || DbExecutor(pool.clone()));

    // Start http server
    server::new(move || {
        App::with_state(AppState{db: addr.clone()})
            // enable logger
            .middleware(middleware::Logger::default())
            // This can be called with:
            // curl --header "Content-Type: application/json" --request POST --data '{"name":"xyz"}'  http://127.0.0.1:8080/add
            .resource("/add2", |r| {
                r.method(http::Method::POST)
                    .with_config(extract_item_limit, |((cfg, _),)| {
                        cfg.limit(4096); // <- limit size of the payload
                    })
            })
            .resource("/add", |r| r.method(http::Method::POST).f(index_add))
            .resource("/add/{name}", |r| r.method(http::Method::GET).with(add))
    }).bind("127.0.0.1:8080")
        .unwrap()
        .start();

    println!("Started http server: 127.0.0.1:8080");
    let _ = sys.run();
}
