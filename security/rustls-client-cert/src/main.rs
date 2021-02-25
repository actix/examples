//! This example shows how to use `actix_web::HttpServer::on_connect` to access client certificates
//! pass them to a handler through request-local data.

use std::{any::Any, env, fs::File, io::BufReader, net::SocketAddr};

use actix_tls::rustls::{ServerConfig, TlsStream};
use actix_web::{
    dev::Extensions, rt::net::TcpStream, web, App, HttpResponse, HttpServer, Responder,
};
use log::info;
use rustls::{
    internal::pemfile::{certs, pkcs8_private_keys},
    AllowAnyAnonymousOrAuthenticatedClient, Certificate, RootCertStore, Session,
};

const CA_CERT: &str = "certs/rootCA.pem";
const SERVER_CERT: &str = "certs/server-cert.pem";
const SERVER_KEY: &str = "certs/server-key.pem";

#[derive(Debug, Clone)]
struct ConnectionInfo {
    bind: SocketAddr,
    peer: SocketAddr,
    ttl: Option<u32>,
}

async fn route_whoami(
    conn_info: web::ReqData<ConnectionInfo>,
    client_cert: Option<web::ReqData<Certificate>>,
) -> impl Responder {
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

        if let Some(mut certs) = tls_session.get_peer_certificates() {
            info!("client certificate found");

            // insert a `rustls::Certificate` into request data
            data.insert(certs.pop().unwrap());
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
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info");
    }

    env_logger::init();

    let mut cert_store = RootCertStore::empty();

    // import CA cert
    let ca_cert = &mut BufReader::new(File::open(CA_CERT)?);
    cert_store
        .add_pem_file(ca_cert)
        .expect("root CA not added to store");

    // set up client authentication requirements
    let client_auth = AllowAnyAnonymousOrAuthenticatedClient::new(cert_store);
    let mut config = ServerConfig::new(client_auth);

    // import server cert and key
    let cert_file = &mut BufReader::new(File::open(SERVER_CERT)?);
    let key_file = &mut BufReader::new(File::open(SERVER_KEY)?);

    let cert_chain = certs(cert_file).unwrap();
    let mut keys = pkcs8_private_keys(key_file).unwrap();
    config.set_single_cert(cert_chain, keys.remove(0)).unwrap();

    // start server
    HttpServer::new(|| App::new().default_service(web::to(route_whoami)))
        .on_connect(get_client_cert)
        .bind(("localhost", 8080))?
        .bind_rustls(("localhost", 8443), config)?
        .workers(1)
        .run()
        .await
}
