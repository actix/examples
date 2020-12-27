use super::init_app_state;
use actix_web::{App, test, http};
use sqlx_user_crud::{controller};
use sqlx_user_crud::model::User;
use uuid::Uuid;
use crate::randomize_string;

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

#[actix_rt::test]
async fn get_user_returns_200_when_user_exists() -> Result<(),sqlx::Error> {
    let app_state = init_app_state().await;
    let mut app = test::init_service(App::new()
        .app_data(app_state.clone())
        .configure(controller::init_user_controller))
        .await;

    let user = User {
        id: Uuid::new_v4().to_string(),
        name: randomize_string("alice"),
        email: randomize_string("alice@email.com"),
        groups: Vec::new(),
    };

    let _ = app_state.context.users.add_user(&user).await?;

    let req = test::TestRequest::get()
        .uri(&format!("/user/{0}", user.id))
        .to_request();

    let resp = test::call_service(&mut app, req).await;
    assert_eq!(resp.status(), http::StatusCode::OK);
    Ok(())
}

#[actix_rt::test]
async fn post_user_returns_200_when_user_is_valid() -> () {
    let app_state = init_app_state().await;
    let mut app = test::init_service(App::new()
        .app_data(app_state.clone())
        .configure(controller::init_user_controller))
        .await;

    let user = User {
        id: Uuid::new_v4().to_string(),
        name: randomize_string("bob"),
        email: randomize_string("bob@email.com"),
        groups: Vec::new(),
    };

    let user = serde_json::to_string(&user).unwrap();
    println!("{0}", &user);

    let req = test::TestRequest::post()
        .uri("user")
        .set_json(&user)
        .to_request();

    let resp = test::call_service(&mut app, req).await;
    assert_eq!(resp.status(), http::StatusCode::ACCEPTED)
}