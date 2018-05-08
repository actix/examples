extern crate actix;
extern crate actix_web;
extern crate env_logger;
extern crate futures;

use actix_web::*;
use futures::Future;

fn index(req: HttpRequest) -> FutureResponse<HttpResponse> {
    req.body()
        .from_err()
        .map(|bytes| HttpResponse::Ok().body(bytes))
        .responder()
}

fn main() {
    ::std::env::set_var("RUST_LOG", "actix_web=error");
    let _ = env_logger::init();
    let sys = actix::System::new("ws-example");

    server::new(|| {
        App::new()
        // enable logger
            .middleware(middleware::Logger::default())
            .resource("/index.html", |r| r.f(|_| "Hello world!"))
            .resource("/", |r| r.f(index))
    }).workers(1)
        .bind("127.0.0.1:8081")
        .unwrap()
        .start();

    println!("Started http server: 127.0.0.1:8081");
    let _ = sys.run();
}
