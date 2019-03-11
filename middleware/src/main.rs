use actix_web::{web, App, HttpServer};

#[allow(dead_code)]
mod redirect;
#[allow(dead_code)]
mod simple;

fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .middleware(redirect::CheckLogin)
            .middleware(simple::SayHi)
            .service(web::resource("/login").to(|| {
                "You are on /login. Go to src/redirect.rs to change this behavior."
            }))
            .service(
                web::resource("/").to(|| {
                    "Hello, middleware! Check the console where the server is run."
                }),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
}
