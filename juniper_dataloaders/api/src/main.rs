extern crate serde_derive;

mod data;
mod db;
mod graphql;
mod type_defs;

use actix_web::{App, HttpServer};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let schema = std::sync::Arc::new(crate::graphql::schema::create_schema());

    let server = HttpServer::new(move || App::new().data(schema.clone()).configure(graphql::route))
        .bind(("0.0.0.0", 8000))
        .unwrap()
        .run();

    eprintln!("Listening on 0.0.0.0:8000");

    server.await
}
