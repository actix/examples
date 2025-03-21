use std::{sync::Arc, time::Instant};

use actix_web::{App, HttpResponse, HttpServer, get, middleware, web::Data};
use awc::{Client, Connector, http::header};
use rustls::{ClientConfig, RootCertStore};

const MAP_URL: &str =
    "https://upload.wikimedia.org/wikipedia/commons/f/ff/Pizigani_1367_Chart_10MB.jpg";

#[get("/")]
async fn fetch_image(client: Data<Client>) -> HttpResponse {
    let start = Instant::now();

    let mut res = client.get(MAP_URL).send().await.unwrap();

    if !res.status().is_success() {
        log::error!("Wikipedia did not return expected image");
        return HttpResponse::InternalServerError().finish();
    }

    let payload = res
        .body()
        // expected image is larger than default body limit
        .limit(20_000_000) // 20MB
        .await
        .unwrap();

    log::info!(
        "it took {}ms to download image to memory",
        start.elapsed().as_millis()
    );

    HttpResponse::Ok()
        .content_type(mime::IMAGE_JPEG)
        .body(payload)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let client_tls_config = Arc::new(rustls_config());

    log::info!("starting HTTP server at http://localhost:8080");

    HttpServer::new(move || {
        // create client _inside_ `HttpServer::new` closure to have one per worker thread
        let client = Client::builder()
            // Wikipedia requires a User-Agent header to make requests
            .add_default_header((header::USER_AGENT, "awc-example/1.0"))
            // a "connector" wraps the stream into an encrypted connection
            .connector(Connector::new().rustls_0_23(Arc::clone(&client_tls_config)))
            .finish();

        App::new()
            .wrap(middleware::Logger::default())
            .app_data(Data::new(client))
            .service(fetch_image)
    })
    .bind(("127.0.0.1", 8080))?
    .workers(2)
    .run()
    .await
}

/// Create simple rustls client config from root certificates.
fn rustls_config() -> ClientConfig {
    rustls::crypto::aws_lc_rs::default_provider()
        .install_default()
        .unwrap();

    let root_store = RootCertStore::from_iter(webpki_roots::TLS_SERVER_ROOTS.to_owned());

    rustls::ClientConfig::builder()
        .with_root_certificates(root_store)
        .with_no_client_auth()
}
