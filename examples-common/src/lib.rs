use tracing::level_filters::LevelFilter;
use tracing_subscriber::{EnvFilter, prelude::*};

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
