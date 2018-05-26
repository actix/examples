/* Actix-Web Asynchronous Database Example

This project illustrates two examples:

    1. An asynchronous handler that executes 4 queries in *sequential order*,
       collecting the results and returning them as a single serialized json object

    2. An asynchronous handler that executes 4 queries in *parallel*,
       collecting the results and returning them as a single serialized json object

*/

#[macro_use] extern crate actix;
extern crate actix_web;
extern crate env_logger;
extern crate failure;
extern crate futures;
extern crate num_cpus;
extern crate r2d2;
extern crate r2d2_sqlite;
extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate serde_json;

use actix::prelude::*;
use actix_web::{
    http, middleware, server, App, AsyncResponder, FutureResponse, HttpResponse,
    State, Error as AWError
};
use std::error::Error as StdError;
use failure::Error;
use futures::future::{Future, join_all, ok as fut_ok, err as fut_err};
use r2d2_sqlite::SqliteConnectionManager;

mod db;
use db::{DbExecutor, Queries, WeatherAgg, Pool}; 

/// State with DbExecutor address
struct AppState {
    db: Addr<Syn, DbExecutor>,
}


/// Version 1: Calls 4 queries in sequential order, as an asynchronous handler
fn asyncio_weather(state: State<AppState>) -> FutureResponse<HttpResponse> {
    let mut result: Vec<Vec<WeatherAgg>> = vec![];

    state.db.send(Queries::GetTopTenHottestYears).from_err()
    .and_then(move |res| {
        result.push(res.unwrap());
        state.db.send(Queries::GetTopTenColdestYears).from_err()
        .and_then(move |res| {
            result.push(res.unwrap());
            state.db.send(Queries::GetTopTenHottestMonths).from_err()
            .and_then(move |res| {
                result.push(res.unwrap());
                state.db.send(Queries::GetTopTenColdestMonths).from_err()
                .and_then(move |res| {
                    result.push(res.unwrap());
                    fut_ok(result)
                    })
            })
        })
    })
    .and_then(|res| Ok(HttpResponse::Ok().json(res)))
    .responder()
}

/// Version 2: Calls 4 queries in parallel, as an asynchronous handler
/// Returning Error types turn into None values in the response
fn parallel_weather(state: State<AppState>) -> FutureResponse<HttpResponse> {
    let fut_result = vec![
        Box::new(state.db.send(Queries::GetTopTenHottestYears)),
        Box::new(state.db.send(Queries::GetTopTenColdestYears)),
        Box::new(state.db.send(Queries::GetTopTenHottestMonths)),
        Box::new(state.db.send(Queries::GetTopTenColdestMonths))];

    join_all(fut_result)
        .map_err(AWError::from)
        .and_then(|result| {
            let res: Vec<Option<Vec<WeatherAgg>>> = 
                result.into_iter().map(|x| x.ok()).collect();

            Ok(HttpResponse::Ok().json(res))
        })
        .responder()
}

fn main() {
    ::std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    let sys = actix::System::new("parallel_db_example");

    // Start N db executor actors (N = number of cores avail)

    let manager = SqliteConnectionManager::file("weather.db");
    let pool = Pool::new(manager).unwrap();

    let addr = SyncArbiter::start(num_cpus::get(), move || DbExecutor(pool.clone()));

    // Start http server
    server::new(move || {
        App::with_state(AppState{db: addr.clone()})
            // enable logger
            .middleware(middleware::Logger::default())
            .resource("/asyncio_weather", |r| 
                r.method(http::Method::GET)
                 .with(asyncio_weather))
            .resource("/parallel_weather", |r| 
                r.method(http::Method::GET)
                 .with(parallel_weather))
    }).bind("127.0.0.1:8080")
        .unwrap()
        .start();

    println!("Started http server: 127.0.0.1:8080");
    let _ = sys.run();
}
