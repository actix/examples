use super::{init_db_context};
use actix_web::{web};
use sqlx_user_crud::{AppState};
use std::sync::{Mutex, Arc};
use actix_web::web::Data;

async fn init_app_state() -> Data<AppState<'static>> {
    let db_context = init_db_context().await;

    web::Data::new(AppState {
        connections: Mutex::new(0),
        context: Arc::new(db_context),
    })
}

#[cfg(test)]
mod index_controller_test;
mod user_controller_test;