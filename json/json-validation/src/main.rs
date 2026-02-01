// This is a contrived example intended to illustrate Actix Web features.
// *Imagine* that you have a process that involves 3 steps.  The steps here
// are dumb in that they do nothing other than call an
// httpbin endpoint that returns the json that was posted to it.  The intent
// here is to illustrate how to chain these steps together as futures and return
// a final result in a response.
//
// Actix Web features illustrated here include:
//     1. handling json input param
//     2. validating user-submitted parameters using the 'validator' crate
//     2. `awc` client features:
//           - POSTing json body
//     3. chaining futures into a single response used by an async endpoint

use std::io;

use actix_web::{
    App, Error, HttpResponse, HttpServer,
    error::ErrorBadRequest,
    web::{self, BytesMut},
};
use awc::Client;
use futures_util::StreamExt as _;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Validate, Deserialize, Serialize)]
struct SomeData {
    #[validate(length(min = 1, max = 1000000))]
    id: String,
    #[validate(length(min = 1, max = 100))]
    name: String,
}

#[allow(dead_code)] // it is debug printed
#[derive(Debug, Deserialize)]
struct HttpBinResponse {
    json: SomeData,
}

/// validate data, post json to httpbin, get it back in the response body, return deserialized
async fn step_x(data: SomeData, client: &Client) -> actix_web::Result<SomeData> {
    // validate data
    data.validate().map_err(ErrorBadRequest)?;

    let mut res = client
        .post("https://httpbin.org/post")
        .send_json(&data)
        .await
        // <- convert SendRequestError to an InternalError, a type that implements the ResponseError trait
        .map_err(actix_web::error::ErrorInternalServerError)?; // <- convert it into an actix_web::Error

    let mut body = BytesMut::new();
    while let Some(chunk) = res.next().await {
        body.extend_from_slice(&chunk?);
    }

    let body = serde_json::from_slice::<HttpBinResponse>(&body).unwrap();

    println!("{body:?}");

    Ok(body.json)
}

async fn create_something(
    some_data: web::Json<SomeData>,
    client: web::Data<Client>,
) -> Result<HttpResponse, Error> {
    let some_data_2 = step_x(some_data.into_inner(), &client).await?;
    let some_data_3 = step_x(some_data_2, &client).await?;
    let d = step_x(some_data_3, &client).await?;

    Ok(HttpResponse::Ok().json(d))
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("starting HTTP server at http://localhost:8080");

    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(Client::default()))
            .service(web::resource("/").route(web::post().to(create_something)))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
