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
async fn post_user_returns_202_when_user_is_valid() -> () {
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

    let req = test::TestRequest::post()
        .uri("/user")
        .set_json(&user)
        .to_request();

    let resp = test::call_service(&mut app, req).await;
    assert_eq!(resp.status(), http::StatusCode::ACCEPTED)
}

#[actix_rt::test]
async fn post_user_returns_202_when_user_and_groups_are_valid() -> Result<(),sqlx::Error> {
    let app_state = init_app_state().await;
    let mut app = test::init_service(App::new()
        .app_data(app_state.clone())
        .configure(controller::init_user_controller))
        .await;

    let group = randomize_string("custodians");
    let _ = app_state.context.groups.add_group(&group).await?;
    let group = app_state.context.groups.get_group_by_name(&group).await?;

    let user = User {
        id: Uuid::new_v4().to_string(),
        name: randomize_string("bob"),
        email: randomize_string("bob@email.com"),
        groups: vec![group]
    };

    let req = test::TestRequest::post()
        .uri("/user")
        .set_json(&user)
        .to_request();

    let resp = test::call_service(&mut app, req).await;
    assert_eq!(resp.status(), http::StatusCode::ACCEPTED);
    Ok(())
}

#[actix_rt::test]
async fn post_user_returns_500_when_user_already_exists() -> Result<(),sqlx::Error> {
    let app_state = init_app_state().await;
    let mut app = test::init_service(App::new()
        .app_data(app_state.clone())
        .configure(controller::init_user_controller))
        .await;

    let user = User {
        id: Uuid::new_v4().to_string(),
        name: randomize_string("charlie"),
        email: randomize_string("charlie@email.com"),
        groups: vec![]
    };

    let _ = app_state.context.users.add_user(&user).await?;

    let req = test::TestRequest::post()
        .uri("/user")
        .set_json(&user)
        .to_request();

    let resp = test::call_service(&mut app, req).await;
    assert_eq!(resp.status(), http::StatusCode::INTERNAL_SERVER_ERROR);
    Ok(())
}

#[actix_rt::test]
async fn patch_user_returns_404_when_user_does_not_exist() -> () {
    let app_state = init_app_state().await;
    let mut app = test::init_service(App::new()
        .app_data(app_state.clone())
        .configure(controller::init_user_controller))
        .await;

    let user = User {
        id: Uuid::new_v4().to_string(),
        name: randomize_string("edison"),
        email: randomize_string("edison@email.com"),
        groups: vec![]
    };

    let req = test::TestRequest::patch()
        .uri("/user")
        .set_json(&user)
        .to_request();

    let resp = test::call_service(&mut app, req).await;
    assert_eq!(resp.status(), http::StatusCode::NOT_FOUND)
}

#[actix_rt::test]
async fn patch_user_returns_202_when_user_exists() -> Result<(),sqlx::Error> {
    let app_state = init_app_state().await;
    let mut app = test::init_service(App::new()
        .app_data(app_state.clone())
        .configure(controller::init_user_controller))
        .await;

    let mut user = User {
        id: Uuid::new_v4().to_string(),
        name: randomize_string("fred"),
        email: randomize_string("fred@email.com"),
        groups: vec![]
    };
    let _ = app_state.context.users.add_user(&user).await?;

    user.name = randomize_string("fredrick");
    let req = test::TestRequest::patch()
        .uri("/user")
        .set_json(&user)
        .to_request();

    let resp = test::call_service(&mut app, req).await;
    assert_eq!(resp.status(), http::StatusCode::ACCEPTED);
    Ok(())
}

#[actix_rt::test]
async fn delete_user_returns_404_when_user_does_not_exist() -> () {
    let app_state = init_app_state().await;
    let mut app = test::init_service(App::new()
        .app_data(app_state.clone())
        .configure(controller::init_user_controller))
        .await;

    let user_id = Uuid::new_v4().to_string();

    let req = test::TestRequest::delete()
        .uri(&format!("/user/{0}", user_id))
        .to_request();

    let resp = test::call_service(&mut app, req).await;
    assert_eq!(resp.status(), http::StatusCode::NOT_FOUND)
}

#[actix_rt::test]
async fn delete_user_returns_200_when_user_exists() -> Result<(),sqlx::Error> {
    let app_state = init_app_state().await;
    let mut app = test::init_service(App::new()
        .app_data(app_state.clone())
        .configure(controller::init_user_controller))
        .await;

    let user = User {
        id: Uuid::new_v4().to_string(),
        name: randomize_string("gina"),
        email: randomize_string("gina@email.com"),
        groups: vec![]
    };
    let _ = app_state.context.users.add_user(&user).await?;

    let req = test::TestRequest::delete()
        .uri(&format!("/user/{0}", &user.id))
        .to_request();

    let resp = test::call_service(&mut app, req).await;
    assert_eq!(resp.status(), http::StatusCode::OK);
    Ok(())
}