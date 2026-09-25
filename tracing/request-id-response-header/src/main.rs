use std::io;

use actix_web::{
    App, HttpMessage, HttpServer,
    dev::Service,
    http::header::{HeaderName, HeaderValue},
    web,
};
use tracing_actix_web::{RequestId, TracingLogger};

async fn hello() -> &'static str {
    "Hello world!"
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    HttpServer::new(move || {
        App::new()
            // set the request id in the `x-request-id` response header
            .wrap_fn(|req, srv| {
                let request_id = req.extensions().get::<RequestId>().copied();
                let res = srv.call(req);
                async move {
                    let mut res = res.await?;
                    if let Some(request_id) = request_id {
                        res.headers_mut().insert(
                            HeaderName::from_static("x-request-id"),
                            // this unwrap never fails, since UUIDs are valid ASCII strings
                            HeaderValue::from_str(&request_id.to_string()).unwrap(),
                        );
                    }
                    Ok(res)
                }
            })
            .wrap(TracingLogger::default())
            .service(web::resource("/hello").to(hello))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await?;

    Ok(())
}
