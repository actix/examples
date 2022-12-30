//! Persistent background jobs using the [`apalis`] crate with a Redis storage backend.

use std::time::Duration;

use apalis::{prelude::*, redis::RedisStorage};
use rand::Rng as _;
use serde::{Deserialize, Serialize};
use tokio::task::JoinHandle;

#[derive(Debug, Deserialize, Serialize)]

pub(crate) struct Email {
    to: String,
}

impl Email {
    pub(crate) fn random() -> Self {
        let user = (&mut rand::thread_rng())
            .sample_iter(rand::distributions::Alphanumeric)
            .take(10)
            .map(char::from)
            .collect::<String>();

        let to = format!("{user}@fake-mail.com");

        Self { to }
    }
}

impl Job for Email {
    const NAME: &'static str = "send_email";
}

async fn process_email_job(job: Email, _ctx: JobContext) -> Result<JobResult, JobError> {
    log::info!("sending email to {}", &job.to);

    // simulate time taken to send email
    tokio::time::sleep(rand_delay_with_jitter()).await;

    Ok(JobResult::Success)
}

pub(crate) async fn start_processing_email_queue() -> anyhow::Result<RedisStorage<Email>> {
    let redis_url = std::env::var("REDIS_URL").expect("Missing env variable REDIS_URL");
    let storage = RedisStorage::connect(redis_url).await?;

    // create job monitor(s) and attach email job handler
    let monitor = Monitor::new().register_with_count(2, {
        let storage = storage.clone();
        move |_n| WorkerBuilder::new(storage.clone()).build_fn(process_email_job)
    });

    // spawn job monitor into background
    let _ = tokio::spawn(async move {
        // run_without_signals: don't listen for ctrl-c because Actix Web does
        // the monitor manages itself otherwise so we don't need to return a join handle
        monitor.run_without_signals().await;
    });

    Ok(storage)
}

/// Returns a duration close to 1 second.
fn rand_delay_with_jitter() -> Duration {
    Duration::from_millis(800_u64 + rand::random::<u8>() as u64 * 2)
}
