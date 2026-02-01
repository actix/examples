//! Actix Web Asynchronous Database Example
//!
//! This project illustrates expensive and blocking database requests that runs
//! in a thread-pool using `web::block` with two examples:
//!
//!     1. An asynchronous handler that executes 4 queries in *sequential order*,
//!        collecting the results and returning them as a single serialized json object
//!
//!     2. An asynchronous handler that executes 4 queries in *parallel*,
//!        collecting the results and returning them as a single serialized json object
//!
//!     Note: The use of sleep(Duration::from_secs(2)); in db.rs is to make performance
//!           improvement with parallelism more obvious.

use std::io;

use actix_web::{middleware, web, App, Error as AWError, HttpResponse, HttpServer};
use futures_util::future::join_all;
use r2d2_sqlite::SqliteConnectionManager;

mod db;
use db::{Pool, Queries};

/// Version 1: Calls 4 queries in sequential order, as an asynchronous handler
async fn asyncio_weather(db: web::Data<Pool>) -> Result<HttpResponse, AWError> {
    let result = vec![
        db::execute(&db, Queries::GetTopTenHottestYears).await?,
        db::execute(&db, Queries::GetTopTenColdestYears).await?,
        db::execute(&db, Queries::GetTopTenHottestMonths).await?,
        db::execute(&db, Queries::GetTopTenColdestMonths).await?,
    ];

    Ok(HttpResponse::Ok().json(result))
}

/// Version 2: Calls 4 queries in parallel, as an asynchronous handler
/// Returning Error types turn into None values in the response
async fn parallel_weather(db: web::Data<Pool>) -> Result<HttpResponse, AWError> {
    let fut_result = vec![
        db::execute(&db, Queries::GetTopTenHottestYears),
        db::execute(&db, Queries::GetTopTenColdestYears),
        db::execute(&db, Queries::GetTopTenHottestMonths),
        db::execute(&db, Queries::GetTopTenColdestMonths),
    ];
    let result: Result<Vec<_>, _> = join_all(fut_result).await.into_iter().collect();

    Ok(HttpResponse::Ok().json(result.map_err(AWError::from)?))
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // connect to SQLite DB
    let manager = SqliteConnectionManager::file("weather.db");
    let pool = Pool::new(manager).unwrap();

    log::info!("starting HTTP server at http://localhost:8080");

    // start HTTP server
    HttpServer::new(move || {
        App::new()
            // store db pool as Data object
            .app_data(web::Data::new(pool.clone()))
            .wrap(middleware::Logger::default())
            .service(web::resource("/asyncio_weather").route(web::get().to(asyncio_weather)))
            .service(web::resource("/parallel_weather").route(web::get().to(parallel_weather)))
    })
    .bind(("127.0.0.1", 8080))?
    .workers(2)
    .run()
    .await
}
