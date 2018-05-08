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
//     3. chaining futures into a single response used by an asynch endpoint

extern crate actix;
extern crate actix_web;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate validator_derive;
extern crate env_logger;
extern crate futures;
extern crate validator;

use actix_web::{client, dev::Handler, http::Method, server, App, AsyncResponder, Body,
                Error, HttpMessage, HttpRequest, HttpResponse, Json, Result};
use futures::{future::ok as fut_ok, Future};
use std::collections::HashMap;
use std::time::Duration;
use validator::{Validate, ValidationError};

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

/// post json to httpbin, get it back in the response body, return deserialized
fn step_x(data: SomeData) -> Box<Future<Item = SomeData, Error = Error>> {
    Box::new(
        client::ClientRequest::post("https://httpbin.org/post")
            .json(data).unwrap()
            .send()
            .conn_timeout(Duration::from_secs(10))
            .map_err(Error::from)   // <- convert SendRequestError to an Error
            .and_then(
                |resp| resp.body()         // <- this is MessageBody type, resolves to complete body
                    .from_err()            // <- convert PayloadError to a Error
                    .and_then(|body| {
                        let resp: HttpBinResponse = serde_json::from_slice(&body).unwrap();
                        fut_ok(resp.json)
                    })
            ),
    )
}

fn create_something(
    some_data: Json<SomeData>,
) -> Box<Future<Item = HttpResponse, Error = Error>> {
    step_x(some_data.into_inner())
        .and_then(|some_data_2| {
            step_x(some_data_2).and_then(|some_data_3| {
                step_x(some_data_3).and_then(|d| {
                    Ok(HttpResponse::Ok()
                        .content_type("application/json")
                        .body(serde_json::to_string(&d).unwrap())
                        .into())
                })
            })
        })
        .responder()
}

fn main() {
    env_logger::init();
    let sys = actix::System::new("asyncio_example");

    server::new(move || {
        App::new().resource("/something", |r| {
            r.method(Method::POST).with(create_something)
        })
    }).bind("127.0.0.1:8088")
        .unwrap()
        .start();

    println!("Started http server: 127.0.0.1:8088");
    let _ = sys.run();
}
