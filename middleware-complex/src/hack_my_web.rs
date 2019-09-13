use actix_web::dev::*;
use actix_web::error::Error;
use actix_web::http;
use futures::{future, prelude::*};

// It makes Middleware. It's Intermediate Object.
pub struct HackMyWeb;

impl<S, B> Transform<S> for HackMyWeb
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = HackMyWebMiddleware<S, B>;
    type Future = future::FutureResult<Self::Transform, Self::InitError>;

    // New Middlware Instance
    fn new_transform(&self, service: S) -> Self::Future {
        future::ok(HackMyWebMiddleware { service })
    }
}

// Middleware Instance.
// In here, you write actual process.
pub struct HackMyWebMiddleware<S, B>
where
    // This is not necessary, but make it easier to understand.
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
{
    service: S,
}

impl<S, B> Service for HackMyWebMiddleware<S, B>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Box<dyn Future<Item = Self::Response, Error = Self::Error>>;

    fn poll_ready(&mut self) -> Result<futures::Async<()>, Self::Error> {
        Ok(futures::Async::Ready(()))
    }

    fn call(&mut self, mut service_request: Self::Request) -> Self::Future {
        // if Request path is "/hack_secret", then overwrite truly secret uri.
        if service_request.path() == "/hack_secret" {
            let secret_uri = "/f18b211dd1744570bb643e800308b1e4"
                .parse::<http::Uri>()
                .unwrap();
            service_request
                .match_info_mut()
                .get_mut()
                .update(&secret_uri);
        }
        Box::new(self.service.call(service_request).map(|mut res| {
            // When this middleware is set, it append my custom header to received response.
            let header_name = http::HeaderName::from_lowercase(b"hacker-code").unwrap();
            let header_value =
                http::HeaderValue::from_str("69de96e2-d5b0-41d4-89b8-864222140e24")
                    .unwrap();
            res.headers_mut().append(header_name, header_value);
            res
        }))
    }
}
