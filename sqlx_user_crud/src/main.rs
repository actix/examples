use std::sync::{Mutex, Arc};
use actix_web::{web, HttpServer, App};
use sqlx_user_crud::dao::DbContext;
use sqlx_user_crud::config::Config;
use sqlx_user_crud::{controller, AppState};
// use crate::controller::user_controller;
// use crate::dao::DbContext;
// use crate::config::Config;

// mod model;
// mod dao;
// mod controller;
// mod config;



// // AppState
// // This the primary dependency for our application's dependency injection.
// // Each controller_test function that interacts with the database will require an `AppState` instance in
// // order to communicate with the database.
// struct AppState<'a> {
//     connections: Mutex<u32>,
//     context: Arc<DbContext<'a>>,
// }

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("=== SQLX User CRUD ===");

    // Read in the configuration file.
    // In small projects this can be a local configuration, but in more sophisticated systems, it is
    // best practice to keep the configuration file on a remote server where it can be retrieved
    // with an http request.
    let config_file: &'static str = "config.json";
    let config = Config::from_file(config_file);
    println!("Using configuration file from {0}", config_file);

    // Connect to the database
    let db_context = DbContext::new(&config.get_database_url()).await;
    println!("Connected to database: {0}", config.get_database_url());

    // Instantiate the app_state. This application state will be cloned for each Actix thread but
    // the Arc of the DbContext will be reused in each Actix thread.
    let app_state = web::Data::new(AppState {
        connections: Mutex::new(0),
        context: Arc::new(db_context),
    });

    // Start the web application.
    // We'll need to transfer ownership of the AppState to the HttpServer via the `move`.
    // Then we can instantiate our controllers.
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
