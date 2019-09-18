// This example is meant to show how to automatically generate a json error response when something goes wrong.

use actix::System;
use actix_web::http::StatusCode;
use actix_web::web::{get, resource, HttpRequest, HttpResponse};
use actix_web::{App, HttpServer, ResponseError};
use futures::future::err;
use futures::Future;
use serde::Serialize;
use serde_json::{json, to_string_pretty};
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::io;

#[derive(Debug, Serialize)]
struct Error {
    msg: String,
    status: u16,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", to_string_pretty(self).unwrap())
    }
}

impl ResponseError for Error {
    // builds the actual response to send back when an error occurs
    fn render_response(&self) -> HttpResponse {
        let err_json = json!({ "error": self.msg });
        HttpResponse::build(StatusCode::from_u16(self.status).unwrap()).json(err_json)
    }
}

fn index(_: HttpRequest) -> impl Future<Item = HttpResponse, Error = Error> {
    err(Error {
        msg: "an example error message".to_string(),
        status: 400,
    })
}

fn main() -> io::Result<()> {
    let sys = System::new("json_error_example");
    let ip_address = "127.0.0.1:8000";

    HttpServer::new(|| App::new().service(resource("/").route(get().to_async(index))))
        .bind(ip_address)
        .expect("Can not bind to port 8000")
        .start();

    println!("Running server on {}", ip_address);

    sys.run()
}
