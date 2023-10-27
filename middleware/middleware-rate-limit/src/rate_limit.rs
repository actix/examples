use std::cell::RefCell;
use std::cmp::min;
use std::future::{ready, Ready};

use actix_web::body::EitherBody;
use actix_web::{
    dev,
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpResponse,
};
use chrono::{Local, NaiveDateTime};
use futures_util::future::LocalBoxFuture;

#[doc(hidden)]
pub struct RateLimitService<S> {
    service: S,
    token_bucket: RefCell<TokenBucket>,
}

impl<S, B> Service<ServiceRequest> for RateLimitService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    dev::forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        log::info!("request is passing through the AddMsg middleware");

        req.uri().path();
        // if be limited
        if !self.token_bucket.borrow_mut().allow_query() {
            return Box::pin(async {
                Ok(req.into_response(
                    HttpResponse::TooManyRequests()
                        .body("")
                        .map_into_right_body(),
                ))
            });
        }

        let fut = self.service.call(req);
        Box::pin(async move { fut.await.map(ServiceResponse::map_into_left_body) })
    }
}

#[derive(Clone, Debug)]
pub struct RateLimit {
    // limit in 10s
    limit: u64,
}

impl RateLimit {
    pub fn new(limit: u64) -> Self {
        Self { limit }
    }
}

impl<S, B> Transform<S, ServiceRequest> for RateLimit
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Transform = RateLimitService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(RateLimitService {
            service,
            token_bucket: RefCell::new(TokenBucket::new(self.limit)),
        }))
    }
}

struct TokenBucket {
    // limit in ten sec
    limit: u64,
    last_query_time: NaiveDateTime,
    // max query number in ten sec,in this case equal limit
    capacity: u64,
    // numbers of token,default equal capacity
    tokens: u64,
}

impl TokenBucket {
    fn new(limit: u64) -> Self {
        TokenBucket {
            limit,
            last_query_time: Default::default(),
            capacity: limit,
            tokens: 0,
        }
    }

    fn allow_query(&mut self) -> bool {
        let current_time = Local::now().naive_local();

        let time_elapsed = (current_time.timestamp() - self.last_query_time.timestamp()) as u64;

        let tokens_to_add = time_elapsed * self.limit / 10;

        self.tokens = min(self.tokens + tokens_to_add, self.capacity);

        if self.tokens > 0 {
            self.last_query_time = current_time;
            self.tokens -= 1;
            true
        } else {
            false
        }
    }
}
