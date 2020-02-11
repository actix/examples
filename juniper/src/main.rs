//! Actix web juniper example
//!
//! A simple example integrating juniper in actix-web
use std::io;
use std::sync::Arc;

use actix_web::{middleware, web, App, Error, HttpResponse, HttpServer};
use juniper_actix::{graphql_handler, playground_handler};

mod schema;

use crate::schema::{create_schema, Schema};

async fn playground() -> Result<HttpResponse, Error> {
    playground_handler("/graphql", None).await
}

async fn graphql(
    req: actix_web::HttpRequest,
    payload: actix_web::web::Payload,
    st: web::Data<Arc<Schema>>,
) -> Result<HttpResponse, Error> {
    graphql_handler(&st, &(), req, payload).await
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    // Create Juniper schema
    let schema = std::sync::Arc::new(create_schema());

    // Start http server
    HttpServer::new(move || {
        App::new()
            .data(schema.clone())
            .wrap(middleware::Logger::default())
            .service(web::resource("/graphql").route(web::post().to(graphql)))
            .service(web::resource("/playground").route(web::get().to(playground)))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
