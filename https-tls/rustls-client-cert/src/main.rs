//! This example shows how to use `actix_web::HttpServer::on_connect` to access client certificates
//! pass them to a handler through connection-local data.

use std::{any::Any, fs::File, io::BufReader, net::SocketAddr, sync::Arc};

use actix_tls::accept::rustls_0_23::TlsStream;
use actix_web::{
    dev::Extensions, rt::net::TcpStream, web, App, HttpRequest, HttpResponse, HttpServer, Responder,
};
use log::info;
use rustls::{
    pki_types::{CertificateDer, PrivateKeyDer},
    server::WebPkiClientVerifier,
    RootCertStore, ServerConfig,
};
use rustls_pemfile::{certs, pkcs8_private_keys};

const CA_CERT: &str = "certs/rootCA.pem";
const SERVER_CERT: &str = "certs/server-cert.pem";
const SERVER_KEY: &str = "certs/server-key.pem";

#[allow(dead_code)] // it is debug printed
#[derive(Debug, Clone)]
struct ConnectionInfo {
    bind: SocketAddr,
    peer: SocketAddr,
    ttl: Option<u32>,
}

async fn route_whoami(req: HttpRequest) -> impl Responder {
    let conn_info = req.conn_data::<ConnectionInfo>().unwrap();
    let client_cert = req.conn_data::<CertificateDer<'static>>();

    if let Some(cert) = client_cert {
        HttpResponse::Ok().body(format!("{:?}\n\n{:?}", &conn_info, &cert))
    } else {
        HttpResponse::Unauthorized().body("No client certificate provided.")
    }
}

fn get_client_cert(connection: &dyn Any, data: &mut Extensions) {
    if let Some(tls_socket) = connection.downcast_ref::<TlsStream<TcpStream>>() {
        info!("TLS on_connect");

        let (socket, tls_session) = tls_socket.get_ref();

        data.insert(ConnectionInfo {
            bind: socket.local_addr().unwrap(),
            peer: socket.peer_addr().unwrap(),
            ttl: socket.ttl().ok(),
        });

        if let Some(certs) = tls_session.peer_certificates() {
            info!("client certificate found");

            // insert a `rustls::Certificate` into request data
            data.insert(certs.last().unwrap().clone());
        }
    } else if let Some(socket) = connection.downcast_ref::<TcpStream>() {
        info!("plaintext on_connect");

        data.insert(ConnectionInfo {
            bind: socket.local_addr().unwrap(),
            peer: socket.peer_addr().unwrap(),
            ttl: socket.ttl().ok(),
        });
    } else {
        unreachable!("socket should be TLS or plaintext");
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    rustls::crypto::aws_lc_rs::default_provider()
        .install_default()
        .unwrap();

    let mut cert_store = RootCertStore::empty();

    // import CA cert
    let ca_cert = &mut BufReader::new(File::open(CA_CERT)?);
    let ca_cert = certs(ca_cert).collect::<Result<Vec<_>, _>>().unwrap();

    for cert in ca_cert {
        cert_store.add(cert).expect("root CA not added to store");
    }

    // set up client authentication requirements
    let client_auth = WebPkiClientVerifier::builder(Arc::new(cert_store))
        .build()
        .unwrap();
    let config = ServerConfig::builder().with_client_cert_verifier(client_auth);

    // import server cert and key
    let cert_file = &mut BufReader::new(File::open(SERVER_CERT)?);
    let key_file = &mut BufReader::new(File::open(SERVER_KEY)?);

    let cert_chain = certs(cert_file).collect::<Result<Vec<_>, _>>().unwrap();
    let mut keys = pkcs8_private_keys(key_file)
        .map(|key| key.map(PrivateKeyDer::Pkcs8))
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    let config = config.with_single_cert(cert_chain, keys.remove(0)).unwrap();

    log::info!("starting HTTP server at http://localhost:8080 and https://localhost:8443");

    HttpServer::new(|| App::new().default_service(web::to(route_whoami)))
        .on_connect(get_client_cert)
        .bind(("localhost", 8080))?
        .bind_rustls_0_23(("localhost", 8443), config)?
        .workers(1)
        .run()
        .await
}
