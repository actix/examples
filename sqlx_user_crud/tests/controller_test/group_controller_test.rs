use super::{init_app_state, randomize_string};
use actix_web::{http, test, App};
use sqlx;
use sqlx_user_crud::controller;
use sqlx_user_crud::controller::group_controller::GroupUpdate;

#[actix_rt::test]
async fn get_group_returns_404_when_not_found() -> () {
    let app_state = init_app_state().await;
    let mut app = test::init_service(
        App::new()
            .app_data(app_state.clone())
            .configure(controller::init_group_controller),
    )
    .await;

    let req = test::TestRequest::get().uri("/group/0").to_request();

    let resp = test::call_service(&mut app, req).await;

    assert_eq!(resp.status(), http::StatusCode::NOT_FOUND);
}

#[actix_rt::test]
async fn post_group_returns_204_when_valid_group_is_added() -> () {
    let app_state = init_app_state().await;
    let mut app = test::init_service(
        App::new()
            .app_data(app_state.clone())
            .configure(controller::init_group_controller),
    )
    .await;

    let group_name = randomize_string("user");

    let req = test::TestRequest::post()
        .uri("/group")
        .set_json(&group_name)
        .to_request();

    let resp = test::call_service(&mut app, req).await;
    assert_eq!(resp.status(), http::StatusCode::ACCEPTED);
}

#[actix_rt::test]
async fn patch_group_returns_204_when_group_is_patched() -> Result<(), sqlx::Error> {
    let app_state = init_app_state().await;
    let mut app = test::init_service(
        App::new()
            .app_data(app_state.clone())
            .configure(controller::init_group_controller),
    )
    .await;

    let group_name = randomize_string("administrator");
    let _ = app_state.context.groups.add_group(&group_name).await?;

    let update = GroupUpdate {
        old: group_name,
        new: randomize_string("Administrator"),
    };

    let req = test::TestRequest::patch()
        .uri("/group")
        .set_json(&update)
        .to_request();

    let resp = test::call_service(&mut app, req).await;
    assert_eq!(resp.status(), http::StatusCode::ACCEPTED);
    Ok(())
}

#[actix_rt::test]
async fn delete_group_returns_200_when_group_is_deleted() -> Result<(), sqlx::Error> {
    let app_state = init_app_state().await;
    let mut app = test::init_service(
        App::new()
            .app_data(app_state.clone())
            .configure(controller::init_group_controller),
    )
    .await;

    let group_name = randomize_string("developers");
    let _ = app_state.context.groups.add_group(&group_name).await?;

    let req = test::TestRequest::delete()
        .uri(&format!("/group/{0}", group_name))
        .to_request();

    let resp = test::call_service(&mut app, req).await;
    assert_eq!(resp.status(), http::StatusCode::OK);
    Ok(())
}
