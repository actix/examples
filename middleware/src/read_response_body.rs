use actix_service::{Service, Transform};
use actix_web::body::{BodySize, MessageBody, ResponseBody};
use std::marker::PhantomData;
use actix_web::{dev::ServiceRequest, dev::ServiceResponse, Error};
use bytes::{Bytes, BytesMut};
use futures::Async;
use futures::future::{ok, FutureResult};
use futures::{Future, Poll};

pub struct Logging;

impl<S: 'static, B> Transform<S> for Logging
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    B: MessageBody + 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<BodyLogger<B>>;
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
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    B: MessageBody,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<BodyLogger<B>>;
    type Error = Error;
    type Future = WrapperStream<S, B>;

    fn poll_ready(&mut self) -> Poll<(), Self::Error> {
        self.service.poll_ready()
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        WrapperStream {
            fut: self.service.call(req),
            _t: PhantomData,
        }
    }
}

pub struct WrapperStream<S, B>
where
    B: MessageBody,
    S: Service,
{
    fut: S::Future,
    _t: PhantomData<(B,)>,
}

impl<S, B> Future for WrapperStream<S, B>
where
    B: MessageBody,
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
{
    type Item = ServiceResponse<BodyLogger<B>>;
    type Error = Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        let res = futures::try_ready!(self.fut.poll());

        Ok(Async::Ready(res.map_body(move |_, body| {
            ResponseBody::Body(BodyLogger {
                body,
                body_accum: BytesMut::new(),
            })
        })))
    }
}

pub struct BodyLogger<B> {
    body: ResponseBody<B>,
    body_accum: BytesMut,
}

impl<B> Drop for BodyLogger<B> {
    fn drop(&mut self) {
        println!("response body: {:?}", self.body_accum);
    }
}

impl<B: MessageBody> MessageBody for BodyLogger<B> {
    fn size(&self) -> BodySize {
        self.body.size()
    }

    fn poll_next(&mut self) -> Poll<Option<Bytes>, Error> {
        match self.body.poll_next()? {
            Async::Ready(Some(chunk)) => {
                self.body_accum.extend_from_slice(&chunk);
                Ok(Async::Ready(Some(chunk)))
            }
            val => Ok(val),
        }
    }
}
