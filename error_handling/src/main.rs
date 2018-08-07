/*
The goal of this example is to show how to propagate a custom error type, derived
from the Fail trait, to a web handler that will evaluate the type of error that
was raised and return an appropriate HTTPResponse.

This example uses a 50/50 chance of returning 200 Ok, otherwise one of four possible
http errors will be chosen, each with an equal chance of being selected: 
    1. 403 Forbidden
    2. 401 Unauthorized
    3. 500 InternalServerError
    4. 400 BadRequest

*/


extern crate actix;
extern crate actix_web;
extern crate env_logger;
#[macro_use] extern crate failure;
extern crate futures;
extern crate rand;


use actix_web::{
    http::Method, server, App, AsyncResponder, Error as ActixWebError,
    HttpResponse, HttpRequest
};
use failure::Error as FailureError;  // naming it clearly for illustration purposes
use futures::{
    future::{
        ok as fut_ok, 
        err as fut_err
    },
    Future
};
use rand::{thread_rng, Rng, distributions::{Distribution, Standard}};



#[derive(Fail, Debug)]
pub enum CustomError {
    #[fail(display = "Custom Error 1")]
    CustomOne,
    #[fail(display = "Custom Error 2")]
    CustomTwo,
    #[fail(display = "Custom Error 3")]
    CustomThree,
    #[fail(display = "Custom Error 4")]
    CustomFour
}


impl Distribution<CustomError> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> CustomError {
        match rng.gen_range(0, 4) {
            0 => CustomError::CustomOne,
            1 => CustomError::CustomTwo,
            2 => CustomError::CustomThree,
            _ => CustomError::CustomFour
        }
    }
}

/*
impl ResponseError for CustomError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR)
    }
}
*/


/// randomly returns either () or one of the 4 CustomError variants
//fn do_something_random() -> impl Future<Item = Result<(), FailureError>,
//                                        Error = ActixWebError> {
fn do_something_random() -> impl Future<Item = (), Error = CustomError> {
    let mut rng = thread_rng();

    // 20% chance that () will be returned by this function
    if rng.gen_bool(2.0/10.0) {
        return fut_ok(())
    }

    let err: CustomError = rand::random();
    return fut_err(err)
}


fn do_something(_req: HttpRequest)
                -> impl Future<Item = HttpResponse, Error = ActixWebError> {

    do_something_random()
    .then(|result| match result {
        Ok(_) => Ok(HttpResponse::Ok()
                    .body("Nothing interesting happened.  Try again.")),

        Err(err) => match err {
              CustomError::CustomOne => {
                  println!("do some stuff related to CustomOne error");
                  Ok(HttpResponse::Forbidden().finish())
              },

              CustomError::CustomTwo => {
                  println!("do some stuff related to CustomTwo error");
                  Ok(HttpResponse::Unauthorized().finish())
              },

              CustomError::CustomThree => {
                  println!("do some stuff related to CustomThree error");
                  Ok(HttpResponse::InternalServerError().finish())
              },
             
              _ => {
                  println!("do some stuff related to CustomFour error");
                  Ok(HttpResponse::BadRequest().finish())
              }
        }
    })
    .responder()
}


fn main() {
    ::std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    let sys = actix::System::new("error_handling_example");

    server::new(move || {
        App::new()
            .resource("/something", |r|
                      r.method(Method::GET)
                       .with_async(do_something))
    }).bind("127.0.0.1:8088")
        .unwrap()
        .start();

    println!("Started http server: 127.0.0.1:8088");
    let _ = sys.run();
}
