use std::future::{ready, Ready};

use actix_web::body::EitherBody;
use actix_web::dev::{self, ServiceRequest, ServiceResponse};
use actix_web::dev::{Service, Transform};
use actix_web::{http, Error, HttpResponse};
use futures::future::LocalBoxFuture;

pub struct CheckLogin;

impl<S, B> Transform<S, ServiceRequest> for CheckLogin
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = CheckLoginMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(CheckLoginMiddleware { service }))
    }
}
pub struct CheckLoginMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for CheckLoginMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    dev::forward_ready!(service);

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
                svc_response.await.map(ServiceResponse::map_into_left_body)
            } else {
                let response = HttpResponse::Found()
                    .insert_header((http::header::LOCATION, "/login"))
                    .finish()
                    .map_into_right_body();

                Ok(ServiceResponse::new(request, response))
            }
        })
    }
}
