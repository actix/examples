use actix_service::{Service, Transform};
use actix_web::dev::{ServiceRequest, ServiceResponse};
use futures::future::{ok, FutureResult};
use futures::{Future, Poll};

// There are two step in middleware processing.
// 1. Middleware initialization, middleware factory get called with
//    next service in chain as parameter.
// 2. Middleware's call method get called with normal request.
pub struct SayHi;

// Middleware factory is `Transform` trait from actix-service crate
// `S` - type of the next service
// `P` - type of request's payload
// `B` - type of response's body
impl<S, P, B> Transform<S> for SayHi
where
    S: Service<Request = ServiceRequest<P>, Response = ServiceResponse<B>>,
    S::Future: 'static,
    S::Error: 'static,
    B: 'static,
{
    type Request = ServiceRequest<P>;
    type Response = ServiceResponse<B>;
    type Error = S::Error;
    type InitError = ();
    type Transform = SayHiMiddleware<S>;
    type Future = FutureResult<Self::Transform, Self::InitError>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(SayHiMiddleware { service })
    }
}

pub struct SayHiMiddleware<S> {
    service: S,
}

impl<S, P, B> Service for SayHiMiddleware<S>
where
    S: Service<Request = ServiceRequest<P>, Response = ServiceResponse<B>>,
    S::Future: 'static,
    S::Error: 'static,
    B: 'static,
{
    type Request = ServiceRequest<P>;
    type Response = ServiceResponse<B>;
    type Error = S::Error;
    type Future = Box<Future<Item = Self::Response, Error = Self::Error>>;

    fn poll_ready(&mut self) -> Poll<(), Self::Error> {
        self.service.poll_ready()
    }

    fn call(&mut self, req: ServiceRequest<P>) -> Self::Future {
        println!("Hi from start. You requested: {}", req.path());

        Box::new(self.service.call(req).and_then(|res| {
            println!("Hi from response");
            Ok(res)
        }))
    }
}
