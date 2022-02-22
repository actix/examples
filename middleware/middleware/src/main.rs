use actix_web::{dev::Service, web, App, HttpServer};
use futures_util::FutureExt as _;

mod read_request_body;
mod read_response_body;
mod redirect;
mod simple;

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
            .wrap_fn(|req, srv| {
                println!("Hi from start. You requested: {}", req.path());

                srv.call(req).map(|res| {
                    println!("Hi from response");
                    res
                })
            })
            .service(web::resource("/login").to(|| async {
                "You are on /login. Go to src/redirect.rs to change this behavior."
            }))
            .service(
                web::resource("/").to(|| async {
                    "Hello, middleware! Check the console where the server is run."
                }),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
