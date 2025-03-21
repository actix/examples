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

#[derive(Debug, Display)]
pub enum CustomError {
    #[display("Custom Error 1")]
    CustomOne,
    #[display("Custom Error 2")]
    CustomTwo,
    #[display("Custom Error 3")]
    CustomThree,
    #[display("Custom Error 4")]
    CustomFour,
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
        match self {
            CustomError::CustomOne => {
                println!("do some stuff related to CustomOne error");
                HttpResponse::Forbidden().finish()
            }

            CustomError::CustomTwo => {
                println!("do some stuff related to CustomTwo error");
                HttpResponse::Unauthorized().finish()
            }

            CustomError::CustomThree => {
                println!("do some stuff related to CustomThree error");
                HttpResponse::InternalServerError().finish()
            }

            _ => {
                println!("do some stuff related to CustomFour error");
                HttpResponse::BadRequest().finish()
            }
        }
    }
}

/// randomly returns either () or one of the 4 CustomError variants
async fn do_something_random() -> Result<(), CustomError> {
    let mut rng = rand::rng();

    // 20% chance that () will be returned by this function
    if rng.random_bool(2.0 / 10.0) {
        Ok(())
    } else {
        Err(rand::random::<CustomError>())
    }
}

async fn do_something() -> Result<HttpResponse, Error> {
    do_something_random().await?;

    Ok(HttpResponse::Ok().body("Nothing interesting happened. Try again."))
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
