use std::{fs::File, io::BufReader, path::Path};

use actix_web::{
    http::header::ContentType, middleware, web, App, HttpRequest, HttpResponse, HttpServer,
};
use log::debug;
use notify::{Event, RecursiveMode, Watcher as _};
use rustls::{pki_types::PrivateKeyDer, ServerConfig};
use rustls_pemfile::{certs, pkcs8_private_keys};
use tokio::sync::mpsc;

#[derive(Debug)]
struct TlsUpdated;

async fn index(req: HttpRequest) -> HttpResponse {
    debug!("{req:?}");

    HttpResponse::Ok().content_type(ContentType::html()).body(
        "<!DOCTYPE html><html><body>\
            <p>Welcome to your TLS-secured homepage!</p>\
        </body></html>",
    )
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> eyre::Result<()> {
    color_eyre::install()?;
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    // signal channel used to notify main event loop of cert/key file changes
    let (reload_tx, mut reload_rx) = mpsc::channel(1);

    let mut file_watcher =
        notify::recommended_watcher(move |res: notify::Result<Event>| match res {
            Ok(ev) => {
                log::info!("files changed: {:?}", ev.paths);
                reload_tx.blocking_send(TlsUpdated).unwrap();
            }
            Err(err) => {
                log::error!("file watch error: {err}");
            }
        })
        .unwrap();

    file_watcher
        .watch(Path::new("cert.pem"), RecursiveMode::NonRecursive)
        .unwrap();
    file_watcher
        .watch(Path::new("key.pem"), RecursiveMode::NonRecursive)
        .unwrap();

    // start HTTP server reload loop
    //
    // loop reloads on TLS changes and exits on normal ctrl-c (etc.) signals
    loop {
        // load TLS cert/key files
        let config = load_rustls_config()?;

        log::info!("starting HTTPS server at https://localhost:8443");

        // start running server but don't await it
        let mut server = HttpServer::new(|| {
            App::new()
                .service(web::resource("/").to(index))
                .wrap(middleware::Logger::default())
        })
        .workers(2)
        .bind_rustls_0_23("127.0.0.1:8443", config)?
        .run();

        // server handle to send signals
        let server_hnd = server.handle();

        tokio::select! {
            // poll server continuously
            res = &mut server => {
                log::info!("server shut down via signal or manual command");
                res?;
                break;
            },

            // receiving a message to reload the server
            Some(_) = reload_rx.recv() => {
                log::info!("TLS cert or key updated");

                // send stop signal; no need to wait for completion signal here
                // since we're about to await the server itself
                drop(server_hnd.stop(true));

                // poll and await server shutdown before
                server.await?;

                // restart loop to reload cert/key files
                continue;
            }
        }
    }

    Ok(())
}

fn load_rustls_config() -> eyre::Result<rustls::ServerConfig> {
    rustls::crypto::aws_lc_rs::default_provider()
        .install_default()
        .unwrap();

    // init server config builder with safe defaults
    let config = ServerConfig::builder().with_no_client_auth();

    // load TLS key/cert files
    let cert_file = &mut BufReader::new(File::open("cert.pem")?);
    let key_file = &mut BufReader::new(File::open("key.pem")?);

    // convert files to key/cert objects
    let cert_chain = certs(cert_file).collect::<Result<Vec<_>, _>>().unwrap();
    let mut keys = pkcs8_private_keys(key_file)
        .map(|key| key.map(PrivateKeyDer::Pkcs8))
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    // exit if no keys could be parsed
    if keys.is_empty() {
        eprintln!("Could not locate PKCS 8 private keys.");
        std::process::exit(1);
    }

    Ok(config.with_single_cert(cert_chain, keys.remove(0))?)
}
