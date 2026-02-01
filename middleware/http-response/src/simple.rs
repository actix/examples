use std::{
    future::{Ready, ready},
    rc::Rc,
};

use actix_web::{
    Error, HttpResponseBuilder,
    dev::{self, Service, ServiceRequest, ServiceResponse, Transform},
    http::{StatusCode, header},
};
use futures_util::future::LocalBoxFuture;
// You can move this struct to a separate file.
// this struct below just for example.
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct HttpData {
    pub data: String,
}
// this implementation is optional
impl Default for HttpData {
    fn default() -> Self {
        Self {
            data: "Hello this is default error message! you need to set Authorization header to get thru this.".to_string(),
        }
    }
}

pub struct ReturnHttpResponse;

impl<S: 'static> Transform<S, ServiceRequest> for ReturnHttpResponse
where
    S: Service<ServiceRequest, Response = ServiceResponse, Error = Error>,
    S::Future: 'static,
{
    type Response = ServiceResponse;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddleware {
            service: Rc::new(service),
        }))
    }
}

pub struct AuthMiddleware<S> {
    // This is special: We need this to avoid lifetime issues.
    service: Rc<S>,
}

impl<S> Service<ServiceRequest> for AuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse, Error = Error> + 'static,
    S::Future: 'static,
{
    type Response = ServiceResponse;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    dev::forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let svc = self.service.clone();

        Box::pin(async move {
            let headers = req.headers();
            let _ = match headers.get("Authorization") {
                Some(e) => e,
                None => {
                    let new_response = HttpResponseBuilder::new(StatusCode::BAD_REQUEST)
                        .insert_header((header::CONTENT_TYPE, "application/json"))
                        .json(HttpData::default());
                    return Ok(ServiceResponse::new(
                        req.request().to_owned(), /* or req.request().clone() */
                        new_response,
                    ));
                }
            };

            let res = svc.call(req).await?;
            Ok(res)
        })
    }
}
