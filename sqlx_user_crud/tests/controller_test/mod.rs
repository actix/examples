use super::{init_db_context, randomize_string};
use actix_web::{web, App, test, HttpRequest, HttpResponse, Error};
use sqlx_user_crud::{AppState, controller};
use std::sync::{Mutex, Arc};
use actix_web::dev::Service;
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