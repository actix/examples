use std::io;

use actix_web::{App, HttpServer};
use actix_web_lab::{extract::ThinData, middleware::from_fn};
use tracing_actix_web::TracingLogger;

mod logging;
mod metric_names;
mod middleware;
mod prometheus;
mod routes;

#[actix_web::main]
async fn main() -> io::Result<()> {
    dotenvy::dotenv().ok();
    logging::init();
    let handle = prometheus::init();

    HttpServer::new(move || {
        App::new()
            .app_data(ThinData(handle.clone()))
            .service(routes::hello)
            .service(routes::sleep)
            .service(routes::metrics)
            .wrap(from_fn(middleware::request_telemetry))
            .wrap(TracingLogger::default())
    })
    .workers(2)
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
