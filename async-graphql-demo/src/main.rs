mod starwars;

use actix_web::{web, App, HttpServer};
use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use async_graphql_actix_web::HandlerBuilder;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        let handler = HandlerBuilder::new(
            Schema::new(starwars::QueryRoot, EmptyMutation, EmptySubscription)
                .data(starwars::StarWars::new()),
        )
        .enable_ui("http://localhost:8080", None)
        .build();

        App::new().service(web::resource("/").to(handler))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
