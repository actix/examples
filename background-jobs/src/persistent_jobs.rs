//! Persistent background jobs using the [`apalis`] crate with a Redis storage backend.

use std::time::Duration;

use apalis::prelude::*;
use apalis_redis::{Config, RedisStorage};
use rand::distr::{Alphanumeric, SampleString as _};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct Email {
    to: String,
}

impl Email {
    pub(crate) fn random() -> Self {
        let user = Alphanumeric.sample_string(&mut rand::rng(), 10);
        let to = format!("{user}@fake-mail.com");
        Self { to }
    }
}

async fn process_email_job(job: Email) {
    log::info!("sending email to {}", &job.to);

    // simulate time taken to send email
    tokio::time::sleep(rand_delay_with_jitter()).await;
}

pub(crate) async fn start_processing_email_queue() -> eyre::Result<RedisStorage<Email>> {
    let redis_url = std::env::var("REDIS_URL").expect("Missing env variable REDIS_URL");
    let conn = apalis_redis::connect(redis_url).await?;
    let config = Config::default().set_namespace("send_email");
    let storage = RedisStorage::new_with_config(conn, config);

    // create unmonitored workers for handling emails
    let worker = WorkerBuilder::new("job-handler")
        .concurrency(2)
        .backend(storage.clone())
        .build_fn(process_email_job);

    #[allow(clippy::let_underscore_future)]
    let _ = tokio::spawn(worker.run());

    Ok(storage)
}

/// Returns a duration close to 1 second.
fn rand_delay_with_jitter() -> Duration {
    Duration::from_millis(800_u64 + rand::random::<u8>() as u64 * 2)
}
