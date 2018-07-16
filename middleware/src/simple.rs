extern crate actix_web;

use actix_web::middleware::{Finished, Middleware, Response, Started};
use actix_web::{HttpRequest, HttpResponse, Result};

// Middleware can get called at three stages during the request/response handling. Below is a
// struct that implements all three of them.
pub struct SayHi;

impl<S> Middleware<S> for SayHi {
    fn start(&self, req: &mut HttpRequest<S>) -> Result<Started> {
        println!("Hi from start. You requested: {}", req.path());
        Ok(Started::Done)
    }

    fn response(
        &self,
        _req: &mut HttpRequest<S>,
        resp: HttpResponse,
    ) -> Result<Response> {
        println!("Hi from response");
        Ok(Response::Done(resp))
    }

    fn finish(&self, _req: &mut HttpRequest<S>, _resp: &HttpResponse) -> Finished {
        println!("Hi from finish");
        Finished::Done
    }
}
