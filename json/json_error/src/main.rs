// This example is meant to show how to automatically generate a json error response when something goes wrong.
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::io;

use actix_web::http::StatusCode;
use actix_web::{web, App, HttpServer, ResponseError};
use serde::Serialize;
use serde_json::{json, to_string_pretty};

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
    fn error_response(&self) -> web::HttpResponse {
        let err_json = json!({ "error": self.msg });
        web::HttpResponse::build(StatusCode::from_u16(self.status).unwrap())
            .json(err_json)
    }
}

async fn index() -> Result<web::HttpResponse, Error> {
    Err(Error {
        msg: "an example error message".to_string(),
        status: 400,
    })
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    let ip_address = "127.0.0.1:8000";
    println!("Running server on {}", ip_address);

    HttpServer::new(|| {
        App::new().service(web::resource("/").route(web::get().to(index)))
    })
    .bind(ip_address)
    .expect("Can not bind to port 8000")
    .run()
    .await
}
