use std::{sync::Arc, time::Duration};

use chrono::Utc;
use tokio::{task::JoinHandle, time::sleep};
use tokio_util::sync::CancellationToken;

use crate::ItemCache;

pub(crate) fn init_item_cache() -> (Arc<ItemCache>, JoinHandle<()>, CancellationToken) {
    // construct empty item cache
    let cache = Arc::new(ItemCache::default());

    // stop signal for cache purge job
    let cache_sweep_cancel = CancellationToken::new();

    // spawn cache purge job
    (
        Arc::clone(&cache),
        tokio::spawn(spawn_cache_sweep(
            Arc::clone(&cache),
            cache_sweep_cancel.clone(),
        )),
        cache_sweep_cancel,
    )
}

async fn spawn_cache_sweep(cache: Arc<ItemCache>, stop_signal: CancellationToken) {
    loop {
        // only _try_ to lock so reads and writes from route handlers do not get blocked
        if let Ok(mut cache) = cache.try_lock() {
            let size = cache.len();

            // purge any cached entries where timestamp is in the past
            cache.retain(|_k, v| *v > Utc::now());

            let removed = size - cache.len();

            if removed > 0 {
                log::info!("removed {removed} cache entries");
            } else {
                log::debug!("cache sweep removed no entries")
            }
        }

        tokio::select! {
            _ = sleep(Duration::from_secs(10)) => {
                continue;
            }

            _ = stop_signal.cancelled() => {
                log::info!("gracefully shutting down cache purge job");
                break;
            }
        };
    }
}
