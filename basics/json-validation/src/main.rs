// This is a contrived example intended to illustrate actix-web features.
// *Imagine* that you have a process that involves 3 steps.  The steps here
// are dumb in that they do nothing other than call an
// httpbin endpoint that returns the json that was posted to it.  The intent
// here is to illustrate how to chain these steps together as futures and return
// a final result in a response.
//
// Actix-web features illustrated here include:
//     1. handling json input param
//     2. validating user-submitted parameters using the 'validator' crate
//     2. actix-web client features:
//           - POSTing json body
//     3. chaining futures into a single response used by an async endpoint

use actix_web::{
    error::ErrorBadRequest,
    http::StatusCode,
    web::{self, BytesMut},
    App, Error, HttpResponse, HttpServer,
};
use awc::Client;
use futures::StreamExt;
use serde::{Deserialize, Serialize};
use std::io;
use validator::Validate;
use validator_derive::Validate;

#[derive(Debug, Validate, Deserialize, Serialize)]
struct SomeData {
    #[validate(length(min = 1, max = 1000000))]
    id: String,
    #[validate(length(min = 1, max = 100))]
    name: String,
}

#[derive(Debug, Deserialize)]
struct HttpBinResponse {
    json: SomeData,
}

/// validate data, post json to httpbin, get it back in the response body, return deserialized
async fn step_x(
    data: SomeData,
    client: &Client,
) -> Result<SomeData, actix_web::error::Error> {
    // validate data
    data.validate().map_err(ErrorBadRequest)?;

    let mut res = client
        .post("https://httpbin.org/post")
        .send_json(&data)
        .await
        // <- convert SendRequestError to an InternalError, a type that implements the ResponseError trait
        .map_err(|e| {
            actix_web::error::InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR)
        })?; // <- convert it into an actix_web::Error

    let mut body = BytesMut::new();
    while let Some(chunk) = res.next().await {
        body.extend_from_slice(&chunk?);
    }

    let body: HttpBinResponse = serde_json::from_slice(&body).unwrap();
    Ok(body.json)
}

async fn create_something(
    some_data: web::Json<SomeData>,
    client: web::Data<Client>,
) -> Result<HttpResponse, Error> {
    let some_data_2 = step_x(some_data.into_inner(), &client).await?;
    let some_data_3 = step_x(some_data_2, &client).await?;
    let d = step_x(some_data_3, &client).await?;

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(serde_json::to_string(&d).unwrap()))
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    let endpoint = "127.0.0.1:8080";

    println!("Starting server at: {:?}", endpoint);
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(Client::default()))
            .service(web::resource("/something").route(web::post().to(create_something)))
    })
    .bind(endpoint)?
    .run()
    .await
}
