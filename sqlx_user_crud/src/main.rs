use std::sync::{Mutex, Arc};
use actix_web::{web, HttpServer, App};
use crate::controller::user_controller;
use crate::dao::DbContext;

mod model;
mod dao;
mod controller;

struct AppState<'a> {
    connections: Mutex<u32>,
    context: Arc<DbContext<'a>>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // TODO: provide a db url type that implements ToString & AsStr
    let mysql_url = "mysql://ecfapp:780765502513ab4575149cfc55fb374ee773536f@localhost/ecf_user_db";

    let db_context = DbContext::new(mysql_url).await;

    let app_state = web::Data::new(AppState {
        connections: Mutex::new(0),
        context: Arc::new(db_context),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .configure(controller::init_user_controller)
            .configure(controller::init_group_controller)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
