use actix_web::{App, HttpResponse, HttpServer, dev::Service, get, http};
use futures_util::future::{self, Either, FutureExt};
use rustls::{
    ServerConfig,
    pki_types::{CertificateDer, PrivateKeyDer, pem::PemObject},
};

#[get("/")]
async fn index() -> String {
    String::from("<html><head><title>FOO BAR</title></head><body><h1>FOO BAR</h1></body></html>")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    rustls::crypto::aws_lc_rs::default_provider()
        .install_default()
        .unwrap();

    let cert_chain = CertificateDer::pem_file_iter("cert.pem")
        .unwrap()
        .flatten()
        .collect();

    let key_der =
        PrivateKeyDer::from_pem_file("key.pem").expect("Could not locate PKCS 8 private keys.");

    let config = ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(cert_chain, key_der)
        .unwrap();

    log::info!(
        "starting HTTP server at http://localhost:80 and HTTPS server on http://localhost:443"
    );

    HttpServer::new(|| {
        App::new()
            .wrap_fn(|sreq, srv| {
                let host = sreq.connection_info().host().to_owned();
                let uri = sreq.uri().to_owned();
                let url = format!("https://{host}{uri}");

                // If the scheme is "https" then it will let other services below this wrap_fn
                // handle the request and if it's "http" then a response with redirect status code
                // will be sent whose "location" header will be same as before, with just "http"
                // changed to "https"

                if sreq.connection_info().scheme() == "https" {
                    Either::Left(srv.call(sreq).map(|res| res))
                } else {
                    println!("An http request has arrived here, i will redirect it to use https");
                    Either::Right(future::ready(Ok(sreq.into_response(
                        HttpResponse::MovedPermanently()
                            .append_header((http::header::LOCATION, url))
                            .finish(),
                    ))))
                }
            })
            .service(index)
    })
    .bind(("127.0.0.1", 80))? // HTTP port
    .bind_rustls_0_23(("127.0.0.1", 443), config)? // HTTPS port
    .run()
    .await
}
