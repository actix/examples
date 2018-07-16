extern crate actix_web;

use actix_web::middleware::{Middleware, Started};
use actix_web::{http, HttpRequest, HttpResponse, Result};

pub struct CheckLogin;

impl<S> Middleware<S> for CheckLogin {
    // We only need to hook into the `start` for this middleware.
    fn start(&self, req: &HttpRequest<S>) -> Result<Started> {
        let is_logged_in = false; // Change this to see the change in outcome in the browser

        if is_logged_in {
            return Ok(Started::Done);
        }

        // Don't forward to /login if we are already on /login
        if req.path() == "/login" {
            return Ok(Started::Done);
        }

        Ok(Started::Response(
            HttpResponse::Found()
                .header(http::header::LOCATION, "/login")
                .finish(),
        ))
    }
}
