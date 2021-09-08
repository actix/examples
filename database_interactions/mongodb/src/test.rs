use actix_web::{
    test::{init_service, read_response, read_response_json, TestRequest},
    web::Bytes,
};
use mongodb::Client;

use super::*;

#[actix_rt::test]
async fn test() {
    let uri = std::env::var("MONGODB_URI")
        .unwrap_or_else(|_| "mongodb://localhost:27017".into());

    let client = Client::with_uri_str(uri).await.expect("failed to connect");

    // Clear any data currently in the users collection.
    client
        .database(DB_NAME)
        .collection::<User>(COLL_NAME)
        .drop(None)
        .await
        .expect("drop collection should succeed");

    let mut app =
        init_service(App::new().data(client).service(add_user).service(get_user)).await;

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

    let response = read_response(&mut app, req).await;
    assert_eq!(response, Bytes::from_static(b"user added"));

    let req = TestRequest::get()
        .uri(&format!("/get_user/{}", &user.username))
        .to_request();

    let response: User = read_response_json(&mut app, req).await;
    assert_eq!(response, user);
}
