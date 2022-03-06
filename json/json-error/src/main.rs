//! This example is meant to show how to automatically generate a json error response when something goes wrong.

use std::{
    fmt::{Display, Formatter, Result as FmtResult},
    io,
};

use actix_web::{http::StatusCode, web, App, HttpResponse, HttpServer, ResponseError};
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
    fn error_response(&self) -> HttpResponse {
        let err_json = json!({ "error": self.msg });
        HttpResponse::build(StatusCode::from_u16(self.status).unwrap()).json(err_json)
    }
}

async fn index() -> Result<HttpResponse, Error> {
    Err(Error {
        msg: "an example error message".to_string(),
        status: 400,
    })
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("starting HTTP server at http://localhost:8080");

    HttpServer::new(|| App::new().service(web::resource("/").route(web::get().to(index))))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
