use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use actix_web::{App, HttpServer, web::Data};
use chrono::{DateTime, Utc};

mod ephemeral_jobs;
mod persistent_jobs;
mod routes;

/// Maps data to its cache expiry time.
pub(crate) type ItemCache = Mutex<HashMap<String, DateTime<Utc>>>;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    color_eyre::install()?;
    dotenvy::dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // background jobs relating to local, disposable tasks
    let (item_cache, cache_sweep_handle, cache_sweep_cancel) = ephemeral_jobs::init_item_cache();

    // background jobs that should be run even if the server is restarted
    let email_sender = persistent_jobs::start_processing_email_queue().await?;

    log::info!("starting HTTP server at http://localhost:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(Data::from(Arc::clone(&item_cache)))
            .app_data(Data::new(email_sender.clone()))
            .service(routes::view_cache)
            .service(routes::cache_item)
            .service(routes::send_email)
            .service(routes::send_email_batch)
    })
    .workers(2)
    .bind(("127.0.0.1", 8080))?
    .run()
    .await?;

    // signal cache sweep task to stop running
    cache_sweep_cancel.cancel();

    // wait for the cache sweep job to exit its loop gracefully
    cache_sweep_handle.await.unwrap();

    log::info!("application successfully shut down gracefully");

    Ok(())
}
