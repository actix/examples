/*
The goal of this example is to show how to propagate a custom error type,
to a web handler that will evaluate the type of error that
was raised and return an appropriate HTTPResponse.

This example uses a 50/50 chance of returning 200 Ok, otherwise one of four possible
http errors will be chosen, each with an equal chance of being selected:
    1. 403 Forbidden
    2. 401 Unauthorized
    3. 500 InternalServerError
    4. 400 BadRequest

*/

use actix_web::{App, Error, HttpResponse, HttpServer, ResponseError, web};
use derive_more::Display;
use rand::{
    Rng,
    distr::{Distribution, StandardUniform},
};
use serde::Serialize;

#[derive(Debug, Display, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CustomError {
    #[display("Access forbidden: insufficient permissions")]
    CustomOne,
    #[display("Authentication required")]
    CustomTwo,
    #[display("Internal server error occurred")]
    CustomThree,
    #[display("Invalid request parameters")]
    CustomFour,
}

#[derive(Serialize)]
struct ErrorResponse {
    code: u16,
    message: String,
    error_type: String,
}

impl Distribution<CustomError> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> CustomError {
        match rng.random_range(0..4) {
            0 => CustomError::CustomOne,
            1 => CustomError::CustomTwo,
            2 => CustomError::CustomThree,
            _ => CustomError::CustomFour,
        }
    }
}

/// Actix Web uses `ResponseError` for conversion of errors to a response
impl ResponseError for CustomError {
    fn error_response(&self) -> HttpResponse {
        let (status_code, error_msg) = match self {
            CustomError::CustomOne => {
                log::error!("Forbidden error: {}", self);
                (403, self.to_string())
            }
            CustomError::CustomTwo => {
                log::error!("Unauthorized error: {}", self);
                (401, self.to_string())
            }
            CustomError::CustomThree => {
                log::error!("Internal server error: {}", self);
                (500, self.to_string())
            }
            CustomError::CustomFour => {
                log::error!("Bad request error: {}", self);
                (400, self.to_string())
            }
        };

        let error_response = ErrorResponse {
            code: status_code,
            message: error_msg,
            error_type: format!("{:?}", self),
        };

        HttpResponse::build(actix_web::http::StatusCode::from_u16(status_code).unwrap())
            .json(error_response)
    }
}

/// randomly returns either () or one of the 4 CustomError variants
async fn do_something_random() -> Result<(), CustomError> {
    let mut rng = rand::rng();
    // 20% chance of success
    const SUCCESS_PROBABILITY: f64 = 0.2;
    if rng.random_bool(SUCCESS_PROBABILITY) {
        log::info!("Random operation succeeded");
        Ok(())
    } else {
        let error = rand::random::<CustomError>();
        log::warn!("Random operation failed with error: {}", error);
        Err(error)
    }
}

async fn do_something() -> Result<HttpResponse, Error> {
    do_something_random().await?;
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "status": "success",
        "message": "Nothing interesting happened. Try again."
    })))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("starting HTTP server at http://localhost:8080");

    HttpServer::new(move || {
        App::new().service(web::resource("/something").route(web::get().to(do_something)))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
