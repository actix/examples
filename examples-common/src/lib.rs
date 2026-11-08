use std::io;

use err_report::Report;
use tokio_util::sync::CancellationToken;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::{EnvFilter, prelude::*};

/// Spawns a task that cancels `shutdown` on SIGINT, SIGTERM, or SIGQUIT on Unix. On other
/// platforms, listens for Ctrl-C.
///
/// Signal errors are logged with tracing and also cancel the token. The task is detached and stops
/// when the Tokio runtime exits. Call this within a Tokio runtime.
pub fn watch_shutdown_signal(shutdown: CancellationToken) {
    tokio::spawn(async move {
        let result: io::Result<()> = async {
            #[cfg(unix)]
            {
                use tokio::signal::unix::{SignalKind, signal};

                let mut interrupt = signal(SignalKind::interrupt())?;
                let mut terminate = signal(SignalKind::terminate())?;
                let mut quit = signal(SignalKind::quit())?;

                tokio::select! {
                    _ = interrupt.recv() => {},
                    _ = terminate.recv() => {},
                    _ = quit.recv() => {},
                }

                Ok(())
            }

            #[cfg(not(unix))]
            tokio::signal::ctrl_c().await
        }
        .await;

        if let Err(err) = result {
            tracing::error!(
                error = %Report::new(&err),
                "Failed to listen for shutdown signals"
            );
        }

        shutdown.cancel();
    });
}

/// Initializes standard tracing subscriber.
pub fn init_standard_logger() {
    tracing_subscriber::registry()
        .with(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .from_env_lossy(),
        )
        .with(tracing_subscriber::fmt::layer().without_time())
        .init();
}

/// Load default rustls provider, to be done once for the process.
pub fn init_rustls_provider() {
    rustls::crypto::aws_lc_rs::default_provider()
        .install_default()
        .unwrap();
}
