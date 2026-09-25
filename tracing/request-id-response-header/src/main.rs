use std::io;

use actix_web::{
    App, Error, HttpMessage, HttpServer,
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    http::header::{HeaderName, HeaderValue},
    middleware::{Next, from_fn},
    web,
};
use tracing_actix_web::{RequestId, TracingLogger};

async fn hello() -> &'static str {
    "Hello world!"
}

async fn request_id_header(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    let request_id = req.extensions().get::<RequestId>().copied();
    let mut res = next.call(req).await?;

    if let Some(request_id) = request_id {
        res.headers_mut().insert(
            HeaderName::from_static("x-request-id"),
            // UUIDs contain only valid ASCII header characters.
            HeaderValue::from_str(&request_id.to_string()).unwrap(),
        );
    }

    Ok(res)
}

#[tokio::main]
async fn main() -> io::Result<()> {
    examples_common::init_standard_logger();

    HttpServer::new(move || {
        App::new()
            .wrap(from_fn(request_id_header))
            .wrap(TracingLogger::default())
            .service(web::resource("/hello").to(hello))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await?;

    Ok(())
}
