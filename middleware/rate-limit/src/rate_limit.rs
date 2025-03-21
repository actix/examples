//! Simple leaky-bucket rate-limiter.

use std::{
    cell::RefCell,
    cmp::min,
    future::{Ready, ready},
};

use actix_web::{
    Error, HttpResponse,
    body::EitherBody,
    dev::{Service, ServiceRequest, ServiceResponse, Transform, forward_ready},
};
use chrono::{DateTime, Utc};
use futures_util::{FutureExt as _, TryFutureExt as _, future::LocalBoxFuture};

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

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        log::info!("request is passing through the AddMsg middleware");

        if !self.token_bucket.borrow_mut().allow_query() {
            // request has been rate limited

            return Box::pin(async {
                Ok(req.into_response(
                    HttpResponse::TooManyRequests()
                        .finish()
                        .map_into_right_body(),
                ))
            });
        }

        self.service
            .call(req)
            .map_ok(ServiceResponse::map_into_left_body)
            .boxed_local()
    }
}

#[derive(Clone, Debug)]
pub struct RateLimit {
    /// Request limit for 10 second period.
    limit: u64,
}

impl RateLimit {
    /// Constructs new rate limiter.
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
    /// Request limit for 10 second period.
    limit: u64,

    /// Max number of requests for 10 second period, in this case equal to limit.
    capacity: u64,

    /// Time that last request was accepted.
    last_req_time: DateTime<Utc>,

    /// Numbers of tokens remaining.
    ///
    /// Initialized equal to capacity.
    tokens: u64,
}

impl TokenBucket {
    /// Constructs new leaky bucket.
    fn new(limit: u64) -> Self {
        TokenBucket {
            limit,
            last_req_time: DateTime::<Utc>::UNIX_EPOCH,
            capacity: limit,
            tokens: 0,
        }
    }

    /// Mutates leaky bucket for accepted request.
    fn allow_query(&mut self) -> bool {
        let current_time = Utc::now();

        let time_elapsed = (current_time.timestamp() - self.last_req_time.timestamp()) as u64;

        let tokens_to_add = time_elapsed * self.limit / 10;

        self.tokens = min(self.tokens + tokens_to_add, self.capacity);

        if self.tokens > 0 {
            self.last_req_time = current_time;
            self.tokens -= 1;
            true
        } else {
            false
        }
    }
}
