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

use actix_web::{web, App, Error, HttpResponse, HttpServer, ResponseError};
use derive_more::Display; // naming it clearly for illustration purposes
use futures::future::{err, ok, Future};
use rand::{
    distributions::{Distribution, Standard},
    thread_rng, Rng,
};

#[derive(Debug, Display)]
pub enum CustomError {
    #[display(fmt = "Custom Error 1")]
    CustomOne,
    #[display(fmt = "Custom Error 2")]
    CustomTwo,
    #[display(fmt = "Custom Error 3")]
    CustomThree,
    #[display(fmt = "Custom Error 4")]
    CustomFour,
}

impl Distribution<CustomError> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> CustomError {
        match rng.gen_range(0, 4) {
            0 => CustomError::CustomOne,
            1 => CustomError::CustomTwo,
            2 => CustomError::CustomThree,
            _ => CustomError::CustomFour,
        }
    }
}

/// Actix web uses `ResponseError` for conversion of errors to a response
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
fn do_something_random() -> impl Future<Item = (), Error = CustomError> {
    let mut rng = thread_rng();

    // 20% chance that () will be returned by this function
    if rng.gen_bool(2.0 / 10.0) {
        ok(())
    } else {
        err(rand::random::<CustomError>())
    }
}

fn do_something() -> impl Future<Item = HttpResponse, Error = Error> {
    do_something_random().from_err().and_then(|_| {
        HttpResponse::Ok().body("Nothing interesting happened.  Try again.")
    })
}

fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(move || {
        App::new().service(
            web::resource("/something").route(web::get().to_async(do_something)),
        )
    })
    .bind("127.0.0.1:8088")?
    .run()
}
