use actix_service::{Service, Transform};
use actix_web::body::{Body, MessageBody, ResponseBody};
use actix_web::error::PayloadError;
use actix_web::{dev::ServiceRequest, dev::ServiceResponse, Error};
use bytes::BytesMut;
use futures::future::{ok, FutureResult};
use futures::stream::Stream;
use futures::{Future, Poll};

pub struct Logging;

impl<S: 'static, B> Transform<S> for Logging
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: MessageBody + 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = LoggingMiddleware<S>;
    type Future = FutureResult<Self::Transform, Self::InitError>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(LoggingMiddleware {
            service,
        })
    }
}

pub struct LoggingMiddleware<S> {
    service: S,
}

impl<S, B> Service for LoggingMiddleware<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>
        + 'static,
    S::Future: 'static,
    B: MessageBody + 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Box<dyn Future<Item = Self::Response, Error = Self::Error>>;

    fn poll_ready(&mut self) -> Poll<(), Self::Error> {
        self.service.poll_ready()
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        Box::new(self.service.call(req).and_then(|res| {
            Ok(res.map_body(|_, body| {
                ResponseBody::Other(Body::Bytes(
                    body.fold(BytesMut::new(), move |mut body, chunk| {
                        body.extend_from_slice(&chunk);
                        Ok::<_, PayloadError>(body)
                    })
                    .and_then(move |bytes| {
                        println!("response body: {:?}", &bytes);
                        ok(bytes.freeze())
                    })
                    .wait()
                    .unwrap(),
                ))
            }))
        }))
    }
}
