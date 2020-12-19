use super::{init_db_context};
use actix_web::{web, App, test, HttpRequest, HttpResponse, Error};
use sqlx_user_crud::{AppState, controller};
use std::sync::{Mutex, Arc};
use actix_web::dev::Service;
//
// async fn init_service() -> impl Service<Request = , Response = HttpResponse, Error = Error> {
//     let db_context = init_db_context().await;
//
//     let app_state = web::Data::new(AppState {
//         connections: Mutex::new(0),
//         context: Arc::new(db_context),
//     });
//
//     test::init_service(App::new()
//         .app_data(app_state.clone())
//         .configure(controller::init_index_controller)
//         .configure(controller::init_user_controller)
//         .configure(controller::init_group_controller)).await
// }

#[cfg(test)]
mod index_controller_test;
mod user_controller_test;