use std::future::Future;
use std::marker::PhantomData;
use std::pin::Pin;
use std::task::{Context, Poll};

use actix_service::{Service, Transform};
use actix_web::body::{BodySize, MessageBody, ResponseBody};
use actix_web::{dev::ServiceRequest, dev::ServiceResponse, Error};
use bytes::{Bytes, BytesMut};
use futures::future::{ok, Ready};

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
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(LoggingMiddleware { service })
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

    fn poll_ready(&mut self, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        WrapperStream {
            fut: self.service.call(req),
            _t: PhantomData,
        }
    }
}

#[pin_project::pin_project]
pub struct WrapperStream<S, B>
where
    B: MessageBody,
    S: Service,
{
    #[pin]
    fut: S::Future,
    _t: PhantomData<(B,)>,
}

impl<S, B> Future for WrapperStream<S, B>
where
    B: MessageBody,
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
{
    type Output = Result<ServiceResponse<BodyLogger<B>>, Error>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let res = futures::ready!(self.project().fut.poll(cx));

        Poll::Ready(res.map(|res| {
            res.map_body(move |_, body| {
                ResponseBody::Body(BodyLogger {
                    body,
                    body_accum: BytesMut::new(),
                })
            })
        }))
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

    fn poll_next(&mut self, cx: &mut Context<'_>) -> Poll<Option<Result<Bytes, Error>>> {
        match self.body.poll_next(cx) {
            Poll::Ready(Some(Ok(chunk))) => {
                self.body_accum.extend_from_slice(&chunk);
                Poll::Ready(Some(Ok(chunk)))
            }
            Poll::Ready(Some(Err(e))) => Poll::Ready(Some(Err(e))),
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Pending => Poll::Pending,
        }
    }
}
