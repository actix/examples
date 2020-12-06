use std::sync::{Mutex, Arc};
use actix_web::{web, HttpServer, App};
use crate::controller::user_controller;
use crate::dao::DbContext;
use crate::config::Config;

mod model;
mod dao;
mod controller;
mod config;

struct AppState<'a> {
    connections: Mutex<u32>,
    context: Arc<DbContext<'a>>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("=== SQLX User CRUD ===");

    let config_file: &'static str = "config.json";
    let config = Config::from_file(config_file);
    println!("Using configuration file from {0}", config_file);

    let db_context = DbContext::new(&config.get_database_url()).await;
    println!("Connected to database: {0}", config.get_database_url());

    let app_state = web::Data::new(AppState {
        connections: Mutex::new(0),
        context: Arc::new(db_context),
    });

    let app = HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .configure(controller::init_user_controller)
            .configure(controller::init_group_controller)
    })
    .bind(config.get_app_url())?;
    println!("Listening on: {0}", config.get_app_url());

    app.run().await
}
