#[macro_use]
extern crate log;

use std::env;

use actix_web::{middleware, web, App, HttpResponse, HttpServer, Responder};
use anyhow::Result;
use dotenv::dotenv;
use sqlx::SqlitePool;

// import todo module (routes and model)
mod todo;

// root (/) handler
async fn index() -> impl Responder {
    HttpResponse::Ok().body(
        r#"
        Welcome to Actix-web with SQLx Todos example.
        Available routes:
        GET /todos -> list of all todos
        POST /todo -> create new todo, example: { "description": "learn actix and sqlx", "done": false }
        GET /todo/{id} -> show one todo with requested id
        PUT /todo/{id} -> update todo with requested id, example: { "description": "learn actix and sqlx", "done": true }
        DELETE /todo/{id} -> delete todo with requested id
        "#
    )
}

#[actix_web::main]
async fn main() -> Result<()> {
    dotenv().ok();
    env_logger::init();

    let database_url =
        env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let host = env::var("HOST").expect("HOST is not set in .env file");
    let port = env::var("PORT")
        .expect("PORT is not set in .env file")
        .parse::<u16>()
        .expect("PORT should be a u16");

    info!("using sqlite database at: {}", &database_url);
    let db_pool = SqlitePool::connect(&database_url).await?;

    // startup connection+schema check
    sqlx::query!("SELECT * FROM todos")
        .fetch_optional(&db_pool)
        .await
        .expect("no connection to database");

    let server = HttpServer::new(move || {
        App::new()
            // pass database pool to application so we can access it inside handlers
            .app_data(web::Data::new(db_pool.clone()))
            .wrap(middleware::Logger::default())
            .route("/", web::get().to(index))
            .configure(todo::init) // init todo routes
    })
    .bind((host, port))?;

    info!("Starting server");
    server.run().await?;

    Ok(())
}
