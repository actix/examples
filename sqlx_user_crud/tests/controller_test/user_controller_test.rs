use super::init_app_state;
use actix_web::{App, web, test, http};
use sqlx_user_crud::{controller, AppState};
use std::sync::{Mutex, Arc};
use actix_web::dev::Service;

#[actix_rt::test]
async fn get_user_returns_err_when_not_found() -> () {
    let app_state = init_app_state().await;
    let mut app = test::init_service(App::new()
        .app_data(app_state.clone())
        .configure(controller::init_user_controller))
        .await;

    let req = test::TestRequest::get()
        .uri("/user/n0t-f0un5")
        .to_request();

    let resp = test::call_service(&mut app, req).await;

    assert_eq!(resp.status(), http::StatusCode::NOT_FOUND);
}