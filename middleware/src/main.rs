extern crate actix;
extern crate actix_web;

use actix_web::{server, App};

mod simple;

fn main() {
    let sys = actix::System::new("middleware-example");

    let _addr = server::new(|| {
        App::new()
            .middleware(simple::SayHi)
            .resource("/index.html", |r| r.f(|_| "Hello, middleware!"))
    }).bind("0.0.0.0:8080")
        .unwrap()
        .start();

    let _ = sys.run();
}
