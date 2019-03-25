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
//
// There are 2 versions in this example, one that uses Boxed Futures and the
// other that uses Impl Future, available since rustc v1.26.

#[macro_use]
extern crate validator_derive;
#[macro_use]
extern crate serde_derive;

use std::collections::HashMap;
use std::io;

use actix_http::client;
use actix_web::web::BytesMut;
use actix_web::{web, App, Error, HttpResponse, HttpServer};
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

// -----------------------------------------------------------------------
// v1 uses Boxed Futures, which were the only option prior to rustc v1.26
// -----------------------------------------------------------------------

/// post json to httpbin, get it back in the response body, return deserialized
fn step_x_v1(data: SomeData) -> Box<Future<Item=SomeData, Error=Error>> {
    let mut connector = client::Connector::new().service();

    Box::new(
        client::ClientRequest::post("https://httpbin.org/post")
            .json(data)
            .unwrap()
            .send(&mut connector)
            .map_err(Error::from) // <- convert SendRequestError to an Error
            .and_then(|resp| {
                resp // <- this is MessageBody type, resolves to complete body
                    .from_err() // <- convert PayloadError to an Error
                    .fold(BytesMut::new(), |mut acc, chunk| {
                        acc.extend_from_slice(&chunk);
                        Ok::<_, Error>(acc)
                    })
                    .map(|body| {
                        let body: HttpBinResponse = serde_json::from_slice(&body).unwrap();
                        body.json
                    })
            }),
    )
}

fn create_something_v1(
    some_data: web::Json<SomeData>,
) -> Box<Future<Item=HttpResponse, Error=Error>> {
    Box::new(step_x_v1(some_data.into_inner()).and_then(|some_data_2| {
        step_x_v1(some_data_2).and_then(|some_data_3| {
            step_x_v1(some_data_3).and_then(|d| {
                Ok(HttpResponse::Ok()
                    .content_type("application/json")
                    .body(serde_json::to_string(&d).unwrap())
                    .into())
            })
        })
    }))
}

// ---------------------------------------------------------------
// v2 uses impl Future, available as of rustc v1.26
// ---------------------------------------------------------------

/// post json to httpbin, get it back in the response body, return deserialized
fn step_x_v2(data: SomeData) -> impl Future<Item=SomeData, Error=Error> {
    let mut connector = client::Connector::new().service();

    client::ClientRequest::post("https://httpbin.org/post")
        .json(data)
        .unwrap()
        .send(&mut connector)
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
        })
}

fn create_something_v2(
    some_data: web::Json<SomeData>,
) -> impl Future<Item=HttpResponse, Error=Error> {
    step_x_v2(some_data.into_inner()).and_then(|some_data_2| {
        step_x_v2(some_data_2).and_then(|some_data_3| {
            step_x_v2(some_data_3).and_then(|d| {
                Ok(HttpResponse::Ok()
                    .content_type("application/json")
                    .body(serde_json::to_string(&d).unwrap())
                    .into())
            })
        })
    })
}

fn main() -> io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .service(
                web::resource("/something_v1")
                    .route(web::post().to(create_something_v1)),
            )
            .service(
                web::resource("/something_v2")
                    .route(web::post().to_async(create_something_v2)),
            )
    })
        .bind("127.0.0.1:8088")?
        .run()
}
