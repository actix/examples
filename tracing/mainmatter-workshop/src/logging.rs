use std::{io, time::Duration};

use opentelemetry::{KeyValue, trace::TracerProvider as _};
use opentelemetry_otlp::{WithExportConfig as _, WithTonicConfig as _};
use opentelemetry_sdk::{
    Resource, runtime,
    trace::{SdkTracerProvider, span_processor_with_async_runtime::BatchSpanProcessor},
};
use tonic::metadata::MetadataMap;
use tracing::level_filters::LevelFilter;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::{EnvFilter, layer::SubscriberExt as _, util::SubscriberInitExt as _};

pub(crate) fn init() -> SdkTracerProvider {
    let app_name = "actix-web-mainmatter-telemetry-workshop-capstone";

    let tracer_provider = opentelemetry_tracer(app_name);
    let tracer = tracer_provider.tracer(app_name);
    let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);

    // we prefer the bunyan formatting layer in this example because it captures
    // span enters and exits by default, making a good way to observe request
    // info like duration when
    let stdout_log = BunyanFormattingLayer::new(app_name.to_owned(), io::stdout);

    tracing_subscriber::registry()
        .with(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .from_env_lossy(),
        )
        .with(telemetry)
        .with(JsonStorageLayer)
        .with(stdout_log)
        .init();

    tracer_provider
}

fn opentelemetry_tracer(app_name: &str) -> SdkTracerProvider {
    let honeycomb_key =
        std::env::var("HONEYCOMB_API_KEY").expect("`HONEYCOMB_API_KEY` should be set in your .env");

    let mut metadata = MetadataMap::with_capacity(1);
    metadata.insert("x-honeycomb-team", honeycomb_key.try_into().unwrap());

    let exporter = opentelemetry_otlp::SpanExporter::builder()
        .with_tonic()
        .with_endpoint("https://api.honeycomb.io/api/traces")
        .with_timeout(Duration::from_secs(5))
        .with_metadata(metadata)
        .build()
        .unwrap();

    let tracer_provider = SdkTracerProvider::builder()
        .with_span_processor(
            BatchSpanProcessor::builder(exporter, runtime::TokioCurrentThread).build(),
        )
        .with_resource(
            Resource::builder()
                .with_attributes([KeyValue::new("service.name", app_name.to_owned())])
                .build(),
        )
        .build();

    opentelemetry::global::set_tracer_provider(tracer_provider.clone());

    tracer_provider
}
