use std::{fs::File, io::BufReader};

use actix_web::{get, web, App, HttpServer, Responder};
use rustls::ServerConfig;
use rustls_pemfile::{certs, private_key};

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {}!", name)
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(greet)
    })
    .bind_rustls_0_22(("127.0.0.1", 8080), load_rustls_config())?
    .run()
    .await
}

fn load_rustls_config() -> ServerConfig {
    // init server config builder with safe defaults
    let config = ServerConfig::builder()
        .with_no_client_auth();

    // load TLS key/cert files
    let cert_file = &mut BufReader::new(File::open("cert.pem").unwrap());
    let key_file = &mut BufReader::new(File::open("key.pem").unwrap());

    // convert files to key/cert objects
    let cert_chain: Vec<_> = certs(cert_file).map(|x| x.unwrap()).collect();
    let key_der = private_key(key_file).unwrap().unwrap();

    config.with_single_cert(cert_chain, key_der).unwrap()
}
