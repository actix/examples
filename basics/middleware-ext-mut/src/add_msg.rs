use std::{
    future::{ready, Future, Ready},
    pin::Pin,
    task::{Context, Poll},
};

use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::Error;

#[derive(Debug, Clone)]
pub struct Msg(pub String);

#[doc(hidden)]
pub struct AddMsgService<S> {
    service: S,
    enabled: bool,
}

impl<S, B> Service for AddMsgService<S>
where
    S: Service<
        Request = ServiceRequest,
        Response = ServiceResponse<B>,
        Error = actix_web::Error,
    >,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Error>>>>;

    fn poll_ready(&mut self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&mut self, req: Self::Request) -> Self::Future {
        println!("request is passing through the AddMsg middleware");

        // get mut HttpRequest from ServiceRequest
        let (request, pl) = req.into_parts();

        if self.enabled {
            // insert data into extensions if enabled
            request
                .extensions_mut()
                .insert(Msg("Hello from Middleware!".to_owned()));
        }

        // construct a new service response
        match ServiceRequest::from_parts(request, pl) {
            Ok(req) => Box::pin(self.service.call(req)),
            Err(_) => Box::pin(ready(Err(Error::from(())))),
        }
    }
}

#[derive(Clone, Debug)]
pub struct AddMsg {
    enabled: bool,
}

impl AddMsg {
    pub fn enabled() -> Self {
        Self { enabled: true }
    }

    pub fn disabled() -> Self {
        Self { enabled: false }
    }
}

impl<S, B> Transform<S> for AddMsg
where
    S: Service<
        Request = ServiceRequest,
        Response = ServiceResponse<B>,
        Error = actix_web::Error,
    >,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;
    type Transform = AddMsgService<S>;
    type InitError = ();

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AddMsgService {
            service,
            enabled: self.enabled,
        }))
    }
}
