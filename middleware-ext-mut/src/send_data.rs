use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::Error;
use futures::future::{err, ok, Future, Ready};
use std::pin::Pin;

#[derive(Debug, Clone)]
pub struct Msg(pub String);

pub struct SendDataService<S> {
    service: S,
}

impl<S, B> Service for SendDataService<S>
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

    fn poll_ready(
        &mut self,
        ctx: &mut core::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&mut self, req: Self::Request) -> Self::Future {
        // get mut HttpRequest from ServiceRequest
        let (request, pl) = req.into_parts();

        // insert data into extensions
        request
            .extensions_mut()
            .insert(Msg(String::from("Hello from Middleware!")));

        // construct a new service response
        match ServiceRequest::from_parts(request, pl) {
            Ok(nq) => Box::pin(self.service.call(nq)),
            Err(_) => Box::pin(err(Error::from(()))),
        }
    }
}

#[derive(Clone, Debug)]
pub struct SendDataFactory;

impl<S, B> Transform<S> for SendDataFactory
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
    type Transform = SendDataService<S>;
    type InitError = ();

    fn new_transform(&self, service: S) -> Self::Future {
        ok(SendDataService { service })
    }
}
