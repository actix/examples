use actix_web::{http, server, App};

#[path = "lib.rs"]
mod template;

fn main() {
    let sys = actix::System::new("example-yarte");

    // start http server
    server::new(move || {
        App::new().resource("/", |r| r.method(http::Method::GET).with(template::index))
    })
    .bind("127.0.0.1:8080")
    .unwrap()
    .start();

    println!("Started http server: 127.0.0.1:8080");
    let _ = sys.run();
}
