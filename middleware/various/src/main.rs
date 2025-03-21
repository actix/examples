use std::time::Duration;

use actix_web::{
    App, Error, HttpServer,
    body::MessageBody,
    dev,
    middleware::{Next, from_fn},
    rt::time,
    web,
};

mod read_request_body;
mod read_response_body;
mod redirect;
mod simple;

async fn timeout_10secs(
    req: dev::ServiceRequest,
    next: Next<impl MessageBody + 'static>,
) -> Result<dev::ServiceResponse<impl MessageBody>, Error> {
    match time::timeout(Duration::from_secs(10), next.call(req)).await {
        Ok(res) => res,
        Err(_err) => Err(actix_web::error::ErrorRequestTimeout("")),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("starting HTTP server at http://localhost:8080");

    HttpServer::new(|| {
        App::new()
            .wrap(redirect::CheckLogin)
            .wrap(read_request_body::Logging)
            .wrap(read_response_body::Logging)
            .wrap(simple::SayHi)
            .wrap(from_fn(timeout_10secs))
            .service(web::resource("/login").to(|body: String| async move {
                println!("request body (handler): {body}");
                "You are on /login. Go to src/redirect.rs to change this behavior."
            }))
            .service(
                web::resource("/").to(|| async {
                    "Hello, middleware! Check the console where the server is run."
                }),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .workers(1)
    .run()
    .await
}
