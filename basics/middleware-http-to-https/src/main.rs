use std::fs::File;
use std::io::BufReader;

use actix_web::dev::Service;
use futures::future::FutureExt;

use actix_web::{get, App, HttpServer};
use actix_web::{http, HttpResponse};
use futures::future;
use futures::future::Either;
use rustls::{Certificate, PrivateKey, ServerConfig};
use rustls_pemfile::{certs, pkcs8_private_keys};

#[get("/")]
async fn index() -> String {
    String::from(
        "<html><head><title>FOO BAR</title></head><body><h1>FOO BAR</h1></body></html>",
    )
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let cert_file = &mut BufReader::new(File::open("cert.pem").unwrap());
    let key_file = &mut BufReader::new(File::open("key.pem").unwrap());

    let cert_chain: Vec<Certificate> = certs(cert_file)
        .unwrap()
        .into_iter()
        .map(Certificate)
        .collect();
    let mut keys: Vec<PrivateKey> = pkcs8_private_keys(key_file)
        .unwrap()
        .into_iter()
        .map(PrivateKey)
        .collect();

    let config = ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth()
        .with_single_cert(cert_chain, keys.remove(0))
        .unwrap();

    HttpServer::new(|| {
        App::new()
            .wrap_fn(|sreq, srv| {
                let host = sreq.connection_info().host().to_owned();
                let uri = sreq.uri().to_owned();
                let url = format!("https://{}{}", host, uri);

                // If the scheme is "https" then it will let other services below this wrap_fn
                // handle the request and if it's "http" then a response with redirect status code
                // will be sent whose "location" header will be same as before, with just "http"
                // changed to "https"
                //
                if sreq.connection_info().scheme() == "https" {
                    Either::Left(srv.call(sreq).map(|res| res))
                } else {
                    println!("An http request has arrived here, i will redirect it to use https");
                    return Either::Right(future::ready(Ok(sreq.into_response(
                        HttpResponse::MovedPermanently()
                            .append_header((http::header::LOCATION, url))
                            .finish(),
                    ))));
                }
            })
            .service(index)
    })
    .bind("0.0.0.0:80")? // Port 80 to listen for http request
    .bind_rustls("0.0.0.0:443", config)? // Port 443 to listen for https request
    .run()
    .await
}
