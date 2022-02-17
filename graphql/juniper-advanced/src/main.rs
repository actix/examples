#[macro_use]
extern crate juniper;

use actix_cors::Cors;
use actix_web::{middleware::Logger, web::Data, App, HttpServer};

mod db;
mod handlers;
mod schemas;

use self::{db::get_db_pool, handlers::register};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let pool = get_db_pool();

    log::info!("starting HTTP server on port 8080");
    log::info!("the GraphiQL interface HTTP server at http://localhost:8080/graphiql");

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(pool.clone()))
            .configure(register)
            .wrap(Cors::permissive())
            .wrap(Logger::default())
    })
    .workers(2)
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
