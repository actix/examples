use actix_web::{
    error, middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer,
};
use bytes::BytesMut;
use futures::{Future, Stream};
use json::JsonValue;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct MyObj {
    name: String,
    number: i32,
}

/// This handler uses json extractor
fn index(item: web::Json<MyObj>) -> HttpResponse {
    println!("model: {:?}", &item);
    HttpResponse::Ok().json(item.0) // <- send response
}

/// This handler uses json extractor with limit
fn extract_item(item: web::Json<MyObj>, req: HttpRequest) -> HttpResponse {
    println!("request: {:?}", req);
    println!("model: {:?}", item);

    HttpResponse::Ok().json(item.0) // <- send json response
}

const MAX_SIZE: usize = 262_144; // max payload size is 256k

/// This handler manually load request payload and parse json object
fn index_manual(
    payload: web::Payload,
) -> impl Future<Item = HttpResponse, Error = Error> {
    // payload is a stream of Bytes objects
    payload
        // `Future::from_err` acts like `?` in that it coerces the error type from
        // the future into the final error type
        .from_err()
        // `fold` will asynchronously read each chunk of the request body and
        // call supplied closure, then it resolves to result of closure
        .fold(BytesMut::new(), move |mut body, chunk| {
            // limit max size of in-memory payload
            if (body.len() + chunk.len()) > MAX_SIZE {
                Err(error::ErrorBadRequest("overflow"))
            } else {
                body.extend_from_slice(&chunk);
                Ok(body)
            }
        })
        // `Future::and_then` can be used to merge an asynchronous workflow with a
        // synchronous workflow
        .and_then(|body| {
            // body is loaded, now we can deserialize serde-json
            let obj = serde_json::from_slice::<MyObj>(&body)?;
            Ok(HttpResponse::Ok().json(obj)) // <- send response
        })
}

/// This handler manually load request payload and parse json-rust
fn index_mjsonrust(pl: web::Payload) -> impl Future<Item = HttpResponse, Error = Error> {
    pl.concat2().from_err().and_then(|body| {
        // body is loaded, now we can deserialize json-rust
        let result = json::parse(std::str::from_utf8(&body).unwrap()); // return Result
        let injson: JsonValue = match result {
            Ok(v) => v,
            Err(e) => json::object! {"err" => e.to_string() },
        };
        Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(injson.dump()))
    })
}

fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            // enable logger
            .wrap(middleware::Logger::default())
            .data(web::JsonConfig::default().limit(4096)) // <- limit size of the payload (global configuration)
            .service(web::resource("/extractor").route(web::post().to(index)))
            .service(
                web::resource("/extractor2")
                    .data(web::JsonConfig::default().limit(1024)) // <- limit size of the payload (resource level)
                    .route(web::post().to_async(extract_item)),
            )
            .service(web::resource("/manual").route(web::post().to_async(index_manual)))
            .service(
                web::resource("/mjsonrust").route(web::post().to_async(index_mjsonrust)),
            )
            .service(web::resource("/").route(web::post().to(index)))
    })
    .bind("127.0.0.1:8080")?
    .run()
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::dev::Service;
    use actix_web::{test, web, App, http};

    #[test]
    fn test_index() -> Result<(), Error>  {
        let app = App::new().route("/", web::post().to(index));
        let mut app = test::init_service(app);

        let req = test::TestRequest::post()
            .uri("/")
            .set_json(&MyObj { name: "my-name".to_owned(), number: 43 })
            .to_request();
        let resp = test::block_on(app.call(req)).unwrap();

        assert_eq!(resp.status(), http::StatusCode::OK);

        let response_body = match resp.response().body().as_ref() {
            Some(actix_web::body::Body::Bytes(bytes)) => bytes,
            _ => panic!("Response error"),
        };

        assert_eq!(response_body, r##"{"name":"my-name","number":43}"##);

        Ok(())
    }
}
