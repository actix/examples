/* Actix-Web Asynchronous Database Example

This project illustrates two examples:

    1. An asynchronous handler that executes 4 queries in *sequential order*,
       collecting the results and returning them as a single serialized json object

    2. An asynchronous handler that executes 4 queries in *parallel*,
       collecting the results and returning them as a single serialized json object

    Note: The use of sleep(Duration::from_secs(2)); in db.rs is to make performance
          improvement with parallelism more obvious.
 */
use std::io;

use actix_web::{middleware, web, App, Error as AWError, HttpResponse, HttpServer};
use futures::future::join_all;
use r2d2_sqlite::{self, SqliteConnectionManager};

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
        Box::pin(db::execute(&db, Queries::GetTopTenHottestYears)),
        Box::pin(db::execute(&db, Queries::GetTopTenColdestYears)),
        Box::pin(db::execute(&db, Queries::GetTopTenHottestMonths)),
        Box::pin(db::execute(&db, Queries::GetTopTenColdestMonths)),
    ];
    let result: Result<Vec<_>, _> = join_all(fut_result).await.into_iter().collect();

    Ok(HttpResponse::Ok().json(result.map_err(AWError::from)?))
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    // Start N db executor actors (N = number of cores avail)
    let manager = SqliteConnectionManager::file("weather.db");
    let pool = Pool::new(manager).unwrap();

    // Start http server
    HttpServer::new(move || {
        App::new()
            // store db pool as Data object
            .data(pool.clone())
            .wrap(middleware::Logger::default())
            .service(
                web::resource("/asyncio_weather").route(web::get().to(asyncio_weather)),
            )
            .service(
                web::resource("/parallel_weather")
                    .route(web::get().to(parallel_weather)),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
