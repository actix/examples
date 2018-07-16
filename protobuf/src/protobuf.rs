use bytes::BytesMut;
use futures::{Future, Poll, Stream};

use bytes::IntoBuf;
use prost::DecodeError as ProtoBufDecodeError;
use prost::EncodeError as ProtoBufEncodeError;
use prost::Message;

use actix_web::dev::HttpResponseBuilder;
use actix_web::error::{Error, PayloadError, ResponseError};
use actix_web::http::header::CONTENT_TYPE;
use actix_web::{HttpMessage, HttpRequest, HttpResponse, Responder};

#[derive(Fail, Debug)]
pub enum ProtoBufPayloadError {
    /// Payload size is bigger than 256k
    #[fail(display = "Payload size is bigger than 256k")]
    Overflow,
    /// Content type error
    #[fail(display = "Content type error")]
    ContentType,
    /// Serialize error
    #[fail(display = "ProtoBud serialize error: {}", _0)]
    Serialize(#[cause] ProtoBufEncodeError),
    /// Deserialize error
    #[fail(display = "ProtoBud deserialize error: {}", _0)]
    Deserialize(#[cause] ProtoBufDecodeError),
    /// Payload error
    #[fail(display = "Error that occur during reading payload: {}", _0)]
    Payload(#[cause] PayloadError),
}

impl ResponseError for ProtoBufPayloadError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            ProtoBufPayloadError::Overflow => HttpResponse::PayloadTooLarge().into(),
            _ => HttpResponse::BadRequest().into(),
        }
    }
}

impl From<PayloadError> for ProtoBufPayloadError {
    fn from(err: PayloadError) -> ProtoBufPayloadError {
        ProtoBufPayloadError::Payload(err)
    }
}

impl From<ProtoBufDecodeError> for ProtoBufPayloadError {
    fn from(err: ProtoBufDecodeError) -> ProtoBufPayloadError {
        ProtoBufPayloadError::Deserialize(err)
    }
}

#[derive(Debug)]
pub struct ProtoBuf<T: Message>(pub T);

impl<T: Message> Responder for ProtoBuf<T> {
    type Item = HttpResponse;
    type Error = Error;

    fn respond_to<S>(self, _: &HttpRequest<S>) -> Result<HttpResponse, Error> {
        let mut buf = Vec::new();
        self.0
            .encode(&mut buf)
            .map_err(|e| Error::from(ProtoBufPayloadError::Serialize(e)))
            .and_then(|()| {
                Ok(HttpResponse::Ok()
                    .content_type("application/protobuf")
                    .body(buf)
                    .into())
            })
    }
}

pub struct ProtoBufMessage<U: Message + Default> {
    fut: Box<Future<Item = U, Error = ProtoBufPayloadError>>,
}

impl<U: Message + Default + 'static> ProtoBufMessage<U> {
    /// Create `ProtoBufMessage` for request.
    pub fn new(req: &HttpRequest) -> Self {
        let fut = req
            .payload()
            .map_err(|e| ProtoBufPayloadError::Payload(e))
            .fold(BytesMut::new(), move |mut body, chunk| {
                body.extend_from_slice(&chunk);
                Ok::<_, ProtoBufPayloadError>(body)
            })
            .and_then(|body| Ok(<U>::decode(&mut body.into_buf())?));

        ProtoBufMessage { fut: Box::new(fut) }
    }
}

impl<U: Message + Default + 'static> Future for ProtoBufMessage<U> where {
    type Item = U;
    type Error = ProtoBufPayloadError;

    fn poll(&mut self) -> Poll<U, ProtoBufPayloadError> {
        self.fut.poll()
    }
}

pub trait ProtoBufResponseBuilder {
    fn protobuf<T: Message>(&mut self, value: T) -> Result<HttpResponse, Error>;
}

impl ProtoBufResponseBuilder for HttpResponseBuilder {
    fn protobuf<T: Message>(&mut self, value: T) -> Result<HttpResponse, Error> {
        self.header(CONTENT_TYPE, "application/protobuf");

        let mut body = Vec::new();
        value
            .encode(&mut body)
            .map_err(|e| ProtoBufPayloadError::Serialize(e))?;
        Ok(self.body(body))
    }
}
