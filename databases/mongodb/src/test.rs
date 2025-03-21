use actix_web::{
    test::{TestRequest, call_and_read_body, call_and_read_body_json, init_service},
    web::Bytes,
};

use super::*;

#[actix_web::test]
#[ignore = "requires MongoDB instance running"]
async fn test() {
    let uri = std::env::var("MONGODB_URI").unwrap_or_else(|_| "mongodb://localhost:27017".into());

    let client = Client::with_uri_str(uri).await.expect("failed to connect");

    // Clear any data currently in the users collection.
    client
        .database(DB_NAME)
        .collection::<User>(COLL_NAME)
        .drop()
        .await
        .expect("drop collection should succeed");

    let app = init_service(
        App::new()
            .app_data(web::Data::new(client))
            .service(add_user)
            .service(get_user),
    )
    .await;

    let user = User {
        first_name: "Jane".into(),
        last_name: "Doe".into(),
        username: "janedoe".into(),
        email: "example@example.com".into(),
    };

    let req = TestRequest::post()
        .uri("/add_user")
        .set_form(&user)
        .to_request();

    let response = call_and_read_body(&app, req).await;
    assert_eq!(response, Bytes::from_static(b"user added"));

    let req = TestRequest::get()
        .uri(&format!("/get_user/{}", &user.username))
        .to_request();

    let response: User = call_and_read_body_json(&app, req).await;
    assert_eq!(response, user);
}
