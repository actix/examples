use std::pin::Pin;
use std::task::{Context, Poll};

use actix_service::{Service, Transform};
use actix_web::body::{BoxBody, MessageBody};
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::{http, Error, HttpResponse};
use futures::future::{ok, Ready};
use futures::{Future};

pub struct CheckLogin;

impl<S> Transform<S, ServiceRequest> for CheckLogin
where
    S: Service<ServiceRequest, Response = ServiceResponse<BoxBody>, Error = Error>,
    S::Future: 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type InitError = ();
    type Transform = CheckLoginMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(CheckLoginMiddleware { service })
    }
}
pub struct CheckLoginMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for CheckLoginMiddleware<S>
where
    B: MessageBody + 'static,
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&self, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        // We only need to hook into the `start` for this middleware.
        let is_logged_in = false; // Change this to see the change in outcome in the browser
        let (request, payload) = req.into_parts();
        let svc_response = self
            .service
            .call(ServiceRequest::from_parts(request.clone(), payload));

        Box::pin(async move {
            // Don't forward to /login if we are already on /login
            if is_logged_in || request.path() == "/login" {
                svc_response.await.map(|r| r.map_into_boxed_body())
            } else {
                let response = HttpResponse::Found()
                    .insert_header((http::header::LOCATION, "/login"))
                    .finish();
                Ok(ServiceResponse::new(request, response))
            }
        })
    }
}
