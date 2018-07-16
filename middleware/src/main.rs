extern crate actix;
extern crate actix_web;

use actix_web::{server, App};

#[allow(dead_code)]
mod redirect;
#[allow(dead_code)]
mod simple;

fn main() {
    let sys = actix::System::new("middleware-example");

    let _addr = server::new(|| {
        App::new()
            .middleware(simple::SayHi)
            // .middleware(redirect::CheckLogin)
            .resource("/login", |r| {
                r.f(|_| "You are on /login. Go to src/redirect.rs to change this behavior.")
            })
            .resource("/", |r| {
                r.f(|_| "Hello, middleware! Check the console where the server is run.")
            })
    }).bind("127.0.0.1:8080")
        .unwrap()
        .start();

    let _ = sys.run();
}
