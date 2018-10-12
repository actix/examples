extern crate actix;
extern crate actix_web;
extern crate env_logger;
extern crate rustls;

use std::fs::File;
use std::io::BufReader;

use actix_web::{http, middleware, server, App, Error, HttpRequest, HttpResponse};
use rustls::internal::pemfile::{certs, rsa_private_keys};
use rustls::{NoClientAuth, ServerConfig};
use server::ServerFlags;

use actix_web::fs::StaticFiles;

/// simple handle
fn index(req: &HttpRequest) -> Result<HttpResponse, Error> {
    println!("{:?}", req);
    Ok(HttpResponse::Ok()
        .content_type("text/plain")
        .body("Welcome!"))
}

fn main() {
    if ::std::env::var("RUST_LOG").is_err() {
        ::std::env::set_var("RUST_LOG", "actix_web=info");
    }
    env_logger::init();
    let sys = actix::System::new("ws-example");

    // load ssl keys
    let mut config = ServerConfig::new(NoClientAuth::new());
    let cert_file = &mut BufReader::new(File::open("cert.pem").unwrap());
    let key_file = &mut BufReader::new(File::open("key.pem").unwrap());
    let cert_chain = certs(cert_file).unwrap();
    let mut keys = rsa_private_keys(key_file).unwrap();
    config.set_single_cert(cert_chain, keys.remove(0)).unwrap();

    // actix acceptor
    let acceptor = server::RustlsAcceptor::with_flags(
        config,
        ServerFlags::HTTP1 | ServerFlags::HTTP2,
    );

    server::new(|| {
        App::new()
            // enable logger
            .middleware(middleware::Logger::default())
            // register simple handler, handle all methods
            .resource("/index.html", |r| r.f(index))
            // with path parameters
            .resource("/", |r| r.method(http::Method::GET).f(|_| {
                HttpResponse::Found()
                    .header("LOCATION", "/index.html")
                    .finish()
            }))
            .handler("/static", StaticFiles::new("static").unwrap())
    }).bind_with("127.0.0.1:8443", move || acceptor.clone())
    .unwrap()
    .start();

    println!("Started http server: 127.0.0.1:8443");
    let _ = sys.run();
}