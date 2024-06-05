use std::array;

use metrics::Unit;
use metrics_exporter_prometheus::{PrometheusBuilder, PrometheusHandle};

use crate::metric_names::*;

pub(crate) fn init() -> PrometheusHandle {
    metrics::describe_histogram!(
        HISTOGRAM_HTTP_REQUEST_DURATION,
        Unit::Seconds,
        "Duration (in seconds) a request took to be processed"
    );

    PrometheusBuilder::new()
        .set_buckets_for_metric(
            metrics_exporter_prometheus::Matcher::Full(HISTOGRAM_HTTP_REQUEST_DURATION.to_owned()),
            &exp_buckets::<28>(0.001), // values from ~0.3ms -> ~33s
        )
        .unwrap()
        .install_recorder()
        .unwrap()
}

fn exp_buckets<const N: usize>(base: f64) -> [f64; N] {
    const RATIO: f64 = 1.5;
    array::from_fn(|i| base * RATIO.powi(i as i32 - 3)).map(|val| (val * 1_e7).round() / 1_e7)
}
