use actix_web::{
    body::{self, MessageBody},
    dev::{ServiceRequest, ServiceResponse},
    http::{
        header::{CacheDirective, HeaderValue, CACHE_CONTROL, CACHE_STATUS, CONTENT_TYPE},
        Method, StatusCode,
    },
    middleware, web, App, Error, HttpResponse, HttpServer, Responder,
};
use actix_web_lab::middleware::{from_fn, Next};
use redis::{Client as RedisClient, Commands, RedisError};
use std::env;

fn fib(n: u64) -> u64 {
    if n <= 1 {
        return n;
    }
    fib(n - 1) + fib(n - 2)
}

async fn an_expensive_function(n: web::Path<u64>) -> impl Responder {
    let result = fib(n.to_owned());
    HttpResponse::Ok().body(result.to_string())
}

#[actix_rt::main]
async fn main() -> std::result::Result<(), std::io::Error> {
    env_logger::init();

    let redis_client =
        redis::Client::open(env::var("REDIS_HOST").unwrap_or("redis://localhost:6379".to_owned()))
            .unwrap();

    let listen_port: String = env::var("LISTEN_PORT").unwrap_or(8080.to_string());
    let listen_address: String = ["0.0.0.0", &listen_port].join(":");

    println!("Server is listening at {}...", listen_address);
    HttpServer::new(move || {
        App::new()
            .wrap(from_fn(cache_middleware))
            .app_data(redis_client.to_owned())
            .service(web::resource("/fibonacci/{n}").route(web::get().to(an_expensive_function)))
            .wrap(middleware::Logger::default())
    })
    .bind(listen_address)?
    .run()
    .await?;

    Ok(())
}

async fn cache_middleware(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    // Defining cache key based on requst path and query string
    let key = format!("{}?{}", req.path(), req.query_string());
    println!("cache key: {key:?}");

    // Get "Cache-Control" request header and get cache directive
    let headers = req.headers().to_owned();
    let cache_directive = match headers.get(CACHE_CONTROL) {
        Some(cache_control_header) => cache_control_header.to_str().unwrap_or(""),
        None => "",
    };

    // Initialize Redis Client from App Data
    let redis_client = req.app_data::<RedisClient>();
    // This should always be Some, so let's unwrap
    let mut redis_conn = redis_client.unwrap().get_connection();
    let redis_ok = redis_conn.is_ok();

    // If Redis connection succeeded and request method is GET
    // Also, if cache directive is not "no-cache" and not "no-store"
    if redis_ok
        && req.method() == Method::GET
        && cache_directive != CacheDirective::NoCache.to_string()
        && cache_directive != CacheDirective::NoStore.to_string()
    {
        // Unwrap the connection
        let redis_conn = redis_conn.as_mut().unwrap();

        // Try to get the cached response by defined key
        let cached_response: Result<Vec<u8>, RedisError> = redis_conn.get(key.to_owned());
        if let Err(e) = cached_response {
            // If cache cannot be deserialized
            println!("cache get error: {}", e);
        } else if cached_response.as_ref().unwrap().is_empty() {
            // If cache body is empty
            println!("cache not found");
        } else {
            // If cache is found
            println!("cache found");

            // Prepare response body
            let res = HttpResponse::new(StatusCode::OK).set_body(cached_response.unwrap());
            let mut res = ServiceResponse::new(req.request().to_owned(), res);

            // Define content-type and headers here
            res.headers_mut()
                .append(CONTENT_TYPE, HeaderValue::from_static("application/json"));
            res.headers_mut()
                .append(CACHE_CONTROL, HeaderValue::from_static("max-age=86400"));
            res.headers_mut()
                .append(CACHE_STATUS, HeaderValue::from_static("hit"));

            return Ok(res);
        }
    }

    // If Redis connection fails or cache could not be found
    // Call the next service
    let res = next.call(req).await?;

    // deconstruct response into parts
    let (req, res) = res.into_parts();
    let (res, body) = res.into_parts();

    // Convert body to Bytes
    let body = body::to_bytes(body).await.ok().unwrap();
    // Use bytes directly for caching instead of converting to a String
    let res_body_enc = body.to_vec();

    // Prepare response body
    let res = res.set_body(res_body_enc.to_owned());
    let mut res = ServiceResponse::new(req.to_owned(), res);

    // If a GET request succeeded and cache directive is not "no-store"
    if req.method() == Method::GET
        && StatusCode::is_success(&res.status())
        && cache_directive != CacheDirective::NoStore.to_string()
    {
        // Define response headers here
        res.headers_mut()
            .append(CACHE_CONTROL, HeaderValue::from_static("max-age=86400"));
        res.headers_mut()
            .append(CACHE_STATUS, HeaderValue::from_static("miss"));

        // If Redis connection succeeded
        if redis_ok {
            // Try to insert the response body into Redis
            let mut redis_conn = redis_conn.unwrap();
            let insert = redis::Cmd::set(key, res_body_enc);
            let insert = insert.query::<String>(&mut redis_conn);

            if let Err(e) = insert {
                // If cache insertion failed
                println!("cache insert error: {}", e);
            } else {
                // This should print "cache insert success: OK"
                println!("cache insert success: {}", insert.unwrap());
            }
        } else if let Err(e) = redis_conn {
            // If Redis connection failed
            println!("RedisError: {}", e);
        }
    } else {
        // If the request method is not "GET" or the operation failed or cache directive is "no-store"
        println!("not inserting cache");
    }
    Ok(res)
}
