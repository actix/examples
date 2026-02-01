use std::{sync::Arc, time::Instant};

use actix_web::{App, HttpResponse, HttpServer, get, middleware, web::ThinData};
use awc::{Client, Connector, http::header};

const MAP_URL: &str =
    "https://upload.wikimedia.org/wikipedia/commons/f/ff/Pizigani_1367_Chart_10MB.jpg";

#[get("/")]
async fn fetch_image(client: ThinData<Client>) -> HttpResponse {
    let start = Instant::now();

    let mut res = client.get(MAP_URL).send().await.unwrap();

    if !res.status().is_success() {
        log::error!("Wikipedia did not return expected image");
        return HttpResponse::InternalServerError().finish();
    }

    let payload = res
        .body()
        // expected image is larger than default body limit, set up a higher
        // limit of 20 MB
        .limit(20_000_000)
        .await
        .unwrap();

    log::info!(
        "It took {}ms to download image to memory",
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

    log::info!("Starting HTTP server at http://localhost:8080");

    HttpServer::new(move || {
        // create client _inside_ `HttpServer::new` closure to have one per worker thread
        let client = Client::builder()
            // Wikipedia requires a User-Agent header to make requests
            .add_default_header((header::USER_AGENT, "awc-example/1.0"))
            // a "connector" wraps the stream into an encrypted connection
            .connector(Connector::new().rustls_0_23(Arc::clone(&client_tls_config)))
            .finish();

        App::new()
            .app_data(ThinData(client))
            .service(fetch_image)
            .wrap(middleware::Logger::default())
    })
    .bind(("127.0.0.1", 8080))?
    .workers(2)
    .run()
    .await
}

/// Create simple `rustls` client config.
fn rustls_config() -> rustls::ClientConfig {
    use rustls_platform_verifier::ConfigVerifierExt as _;

    rustls::crypto::aws_lc_rs::default_provider()
        .install_default()
        .unwrap();

    // The benefits of the platform verifier are clear; see:
    // https://github.com/rustls/rustls-platform-verifier#readme
    rustls::ClientConfig::with_platform_verifier()
}
