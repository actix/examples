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

#[macro_use]
extern crate validator_derive;
#[macro_use]
extern crate serde_derive;

use std::collections::HashMap;
use std::io;

use actix_web::{
    client::Client,
    error::ErrorBadRequest,
    web::{self, BytesMut},
    App, Error, HttpResponse, HttpServer,
};
use futures::{Future, Stream};
use validator::Validate;

#[derive(Debug, Validate, Deserialize, Serialize)]
struct SomeData {
    #[validate(length(min = "1", max = "1000000"))]
    id: String,
    #[validate(length(min = "1", max = "100"))]
    name: String,
}

#[derive(Debug, Deserialize)]
struct HttpBinResponse {
    args: HashMap<String, String>,
    data: String,
    files: HashMap<String, String>,
    form: HashMap<String, String>,
    headers: HashMap<String, String>,
    json: SomeData,
    origin: String,
    url: String,
}

/// validate data, post json to httpbin, get it back in the response body, return deserialized
fn step_x(
    data: SomeData,
    client: &Client,
) -> impl Future<Item = SomeData, Error = Error> {
    let validation = futures::future::result(data.validate()).map_err(ErrorBadRequest);
    let post_response = client
        .post("https://httpbin.org/post")
        .send_json(&data)
        .map_err(Error::from) // <- convert SendRequestError to an Error
        .and_then(|resp| {
            resp.from_err()
                .fold(BytesMut::new(), |mut acc, chunk| {
                    acc.extend_from_slice(&chunk);
                    Ok::<_, Error>(acc)
                })
                .map(|body| {
                    let body: HttpBinResponse = serde_json::from_slice(&body).unwrap();
                    body.json
                })
        });

    validation.and_then(|_| post_response)
}

fn create_something(
    some_data: web::Json<SomeData>,
    client: web::Data<Client>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    step_x(some_data.into_inner(), &client).and_then(move |some_data_2| {
        step_x(some_data_2, &client).and_then(move |some_data_3| {
            step_x(some_data_3, &client).and_then(|d| {
                Ok(HttpResponse::Ok()
                    .content_type("application/json")
                    .body(serde_json::to_string(&d).unwrap()))
            })
        })
    })
}

fn main() -> io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    let endpoint = "127.0.0.1:8080";

    println!("Starting server at: {:?}", endpoint);
    HttpServer::new(|| {
        App::new().data(Client::default()).service(
            web::resource("/something").route(web::post().to_async(create_something)),
        )
    })
    .bind(endpoint)?
    .run()
}
