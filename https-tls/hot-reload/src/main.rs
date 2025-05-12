use std::path::Path;

use actix_web::{
    App, HttpRequest, HttpResponse, HttpServer, http::header::ContentType, middleware, web,
};
use eyre::WrapErr as _;
use notify::{Event, RecursiveMode, Watcher as _};
use rustls::{
    ServerConfig,
    pki_types::{CertificateDer, PrivateKeyDer, pem::PemObject},
    sign::CertifiedKey,
};

async fn index(req: HttpRequest) -> HttpResponse {
    tracing::debug!("{req:?}");

    HttpResponse::Ok().content_type(ContentType::html()).body(
        "<!DOCTYPE html><html><body>\
            <p>Welcome to your TLS-secured homepage!</p>\
        </body></html>",
    )
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> eyre::Result<()> {
    color_eyre::install()?;
    examples_common::init_standard_logger();
    examples_common::init_rustls_provider();

    // initial load of certificate and key files
    let cert_key = load_certified_key()?;

    // signal channel used to notify rustls of cert/key file changes
    let (reload_tx, cert_resolver) = rustls_channel_resolver::channel::<8>(cert_key);

    let rustls_config = ServerConfig::builder()
        .with_no_client_auth()
        .with_cert_resolver(cert_resolver);

    // unsupervised watcher thread which will just shutdown when the server stops
    tracing::debug!("Setting up cert watcher");

    let mut file_watcher =
        notify::recommended_watcher(move |res: notify::Result<Event>| match res {
            Ok(ev) => {
                tracing::info!("files changed: {:?}", ev.paths);

                let cert_key = load_certified_key().unwrap();
                reload_tx.update(cert_key);
            }
            Err(err) => {
                tracing::error!("file watch error: {err}");
            }
        })
        .wrap_err("Failed to set up file watcher")?;

    file_watcher
        .watch(Path::new("cert.pem"), RecursiveMode::NonRecursive)
        .wrap_err("Failed to watch cert file")?;
    file_watcher
        .watch(Path::new("key.pem"), RecursiveMode::NonRecursive)
        .wrap_err("Failed to watch key file")?;

    tracing::info!("Starting HTTPS server at https://localhost:8443");

    // start running server as normal (as opposed to in a loop like the cert-watch example)
    HttpServer::new(|| {
        App::new()
            .service(web::resource("/").to(index))
            .wrap(middleware::Logger::default().log_target("@"))
    })
    .workers(2)
    .bind_rustls_0_23(("127.0.0.1", 8443), rustls_config)?
    .run()
    .await?;

    Ok(())
}

fn load_certified_key() -> eyre::Result<rustls::sign::CertifiedKey> {
    // load TLS key/cert files
    let cert_chain = CertificateDer::pem_file_iter("cert.pem")
        .wrap_err("Could not locate certificate chain file")?
        .flatten()
        .collect();

    // load TLS private key file
    let key =
        PrivateKeyDer::from_pem_file("key.pem").wrap_err("Could not locate PKCS 8 private keys")?;

    // parse private key by crypto provider
    let key = rustls::crypto::aws_lc_rs::sign::any_supported_type(&key)
        .wrap_err("Private key type is unsupported")?;

    Ok(CertifiedKey::new(cert_chain, key))
}
