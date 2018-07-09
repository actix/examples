extern crate actix_web;

use actix_web::middleware::{Middleware, Started};
use actix_web::{HttpRequest, Result};

pub struct SayHi;

impl<S> Middleware<S> for SayHi {
    fn start(&self, _req: &mut HttpRequest<S>) -> Result<Started> {
        println!("Hi");
        Ok(Started::Done)
    }
}
