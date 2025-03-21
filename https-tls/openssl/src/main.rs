use std::{
    fs::File,
    io::{self, Read as _},
};

use actix_web::{App, Error, HttpRequest, HttpResponse, HttpServer, middleware, web};
use openssl::{
    pkey::{PKey, Private},
    ssl::{SslAcceptor, SslMethod},
};

/// simple handle
async fn index(req: HttpRequest) -> Result<HttpResponse, Error> {
    println!("{req:?}");
    Ok(HttpResponse::Ok()
        .content_type("text/plain")
        .body("Hello HTTPS World!"))
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // build TLS config from files
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();

    // set the encrypted private key
    builder
        .set_private_key(&load_encrypted_private_key())
        .unwrap();

    // set the unencrypted private key
    // (uncomment if you generate your own key+cert with `mkcert`, and also remove the statement above)
    // builder
    //     .set_private_key_file("key.pem", openssl::ssl::SslFiletype::PEM)
    //     .unwrap();

    // set the certificate chain file location
    builder.set_certificate_chain_file("cert.pem").unwrap();

    log::info!("starting HTTPS server at http://localhost:8443");

    HttpServer::new(|| {
        App::new()
            // enable logger
            .wrap(middleware::Logger::default())
            // simple root handler
            .service(web::resource("/").route(web::get().to(index)))
    })
    .bind_openssl("127.0.0.1:8443", builder)?
    .workers(2)
    .run()
    .await
}

fn load_encrypted_private_key() -> PKey<Private> {
    let mut file = File::open("key.pem").unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).expect("Failed to read file");

    PKey::private_key_from_pem_passphrase(&buffer, b"password").unwrap()
}
