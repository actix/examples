use bytes::BytesMut;
use futures::{Future, Poll, Stream};

use bytes::{Bytes, IntoBuf};
use derive_more::{Display, From};
use prost::DecodeError as ProtoBufDecodeError;
use prost::EncodeError as ProtoBufEncodeError;
use prost::Message;

use actix_web::dev::HttpResponseBuilder;
use actix_web::error::{Error, PayloadError, ResponseError};
use actix_web::http::header::CONTENT_TYPE;
use actix_web::{HttpRequest, HttpResponse, Responder};

#[derive(Debug, Display, From)]
pub enum ProtoBufPayloadError {
    /// Payload size is bigger than 256k
    #[display(fmt = "Payload size is bigger than 256k")]
    Overflow,
    /// Content type error
    #[display(fmt = "Content type error")]
    ContentType,
    /// Serialize error
    #[display(fmt = "ProtoBud serialize error: {}", _0)]
    Serialize(ProtoBufEncodeError),
    /// Deserialize error
    #[display(fmt = "ProtoBud deserialize error: {}", _0)]
    Deserialize(ProtoBufDecodeError),
    /// Payload error
    #[display(fmt = "Error that occur during reading payload: {}", _0)]
    Payload(PayloadError),
}

impl ResponseError for ProtoBufPayloadError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            ProtoBufPayloadError::Overflow => HttpResponse::PayloadTooLarge().into(),
            _ => HttpResponse::BadRequest().into(),
        }
    }
}

#[derive(Debug)]
pub struct ProtoBuf<T: Message>(pub T);

impl<T: Message> Responder for ProtoBuf<T> {
    type Error = Error;
    type Future = Result<HttpResponse, Error>;

    fn respond_to(self, _: &HttpRequest) -> Result<HttpResponse, Error> {
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
    pub fn new<S>(pl: S) -> Self
    where
        S: Stream<Item = Bytes, Error = PayloadError> + 'static,
    {
        let fut = pl
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
