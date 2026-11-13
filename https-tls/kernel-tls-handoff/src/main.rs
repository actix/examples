//! Demonstrates Linux kernel TLS with a Rustls handshake and an Actix HTTP service.

use std::{io, sync::Arc, time::Duration};

use actix_http::{Error, HttpService, Protocol, Request, Response, error::DispatchError};
use actix_service::{ServiceFactoryExt, fn_service};
use ktls::{CorkStream, config_ktls_server};
use rustls::pki_types::PrivatePkcs8KeyDer;
use tokio::net::TcpStream;
use tokio_rustls::TlsAcceptor;
use tokio_util::future::FutureExt as _;

#[tokio::main(flavor = "local")]
async fn main() -> io::Result<()> {
    examples_common::init_standard_logger();

    let acceptor = TlsAcceptor::from(Arc::new(rustls_config()?));

    tracing::info!("Starting kTLS HTTP server at https://localhost:8443");

    actix_server::Server::build()
        .bind("ktls", ("127.0.0.1", 8443), move || {
            let acceptor = acceptor.clone();

            fn_service(move |socket: TcpStream| {
                let acceptor = acceptor.clone();

                async move {
                    let peer_addr = socket.peer_addr().ok();

                    // Bound both the handshake and the drain before handing off to kTLS.
                    let stream = async {
                        // CorkStream preserves TLS record boundaries during the handoff.
                        let tls = acceptor.accept(CorkStream::new(socket)).await?;

                        config_ktls_server(tls).await.map_err(io::Error::other)
                    }
                    .timeout(Duration::from_secs(5))
                    .await
                    .map_err(io::Error::other)??;

                    tracing::info!(?peer_addr, "Kernel TLS enabled for transmit and receive");

                    Ok::<_, io::Error>((stream, Protocol::Http1, peer_addr))
                }
            })
            .map_err(|err| {
                tracing::error!(
                    error = %err_report::Report::new(&err),
                    "TLS handshake or kTLS setup failed"
                );
                DispatchError::Io(err)
            })
            .and_then(HttpService::build().secure().finish(|_req: Request| async {
                Ok::<_, Error>(Response::ok().set_body("Hello from Actix over kernel TLS!\n"))
            }))
        })?
        .run()
        .await
}

fn rustls_config() -> io::Result<rustls::ServerConfig> {
    use rustls::crypto::aws_lc_rs::cipher_suite::TLS_ECDHE_ECDSA_WITH_AES_128_GCM_SHA256;

    let rcgen::CertifiedKey { cert, signing_key } =
        rcgen::generate_simple_self_signed(["localhost".to_owned()]).map_err(io::Error::other)?;

    let mut provider = rustls::crypto::aws_lc_rs::default_provider();

    // Select AES-128-GCM explicitly so Rustls negotiates a cipher supported by kTLS.
    provider.cipher_suites = vec![TLS_ECDHE_ECDSA_WITH_AES_128_GCM_SHA256];

    let mut config = rustls::ServerConfig::builder_with_provider(Arc::new(provider))
        .with_protocol_versions(&[&rustls::version::TLS12])
        .map_err(io::Error::other)?
        .with_no_client_auth()
        .with_single_cert(
            vec![cert.der().clone()],
            PrivatePkcs8KeyDer::from(signing_key.serialize_der()).into(),
        )
        .map_err(io::Error::other)?;

    // kTLS needs the negotiated traffic keys after Rustls completes the handshake.
    config.enable_secret_extraction = true;
    config.alpn_protocols = vec![b"http/1.1".to_vec()];

    Ok(config)
}
