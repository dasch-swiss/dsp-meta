use std::env;
use std::path::Path;
use std::sync::Arc;
use std::time::Duration;

use config::Config;
use dsp_meta::app_state::AppState;
use dsp_meta::domain::metadata_repository::MetadataRepository;
use dsp_meta::domain::metadata_service::MetadataService;
use pid1::Pid1Settings;
use tokio::net::TcpListener;
use opentelemetry::trace::TracerProvider as _;
use opentelemetry::KeyValue;
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::trace::{BatchSpanProcessor, TracerProvider};
use opentelemetry_sdk::Resource;
use tracing::info;
use tracing_subscriber::prelude::*;
use tracing_subscriber::{fmt, EnvFilter};
use url::Url;

fn main() {
    // Do the pid1 magic. Needs to be the first thing executed.
    Pid1Settings::new()
        .enable_log(true)
        .timeout(Duration::from_secs(2))
        .launch()
        .expect("pid1 launch");

    // Manually create a tokio runtime (as opposed to using the macro)
    // We need to do this before initializing OpenTelemetry, as the OTLP exporter
    // requires a runtime context
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();

    // Initialize OpenTelemetry within the runtime context
    let _guard = runtime.enter();

    // Initialize OpenTelemetry tracer provider
    let resource = Resource::new(vec![KeyValue::new("service.name", "dsp-meta")]);

    // Build tracer provider with optional OTLP exporter
    let mut tracer_provider_builder = TracerProvider::builder()
        .with_config(opentelemetry_sdk::trace::Config::default().with_resource(resource));

    // If OTLP endpoint is configured, add batch exporter
    if let Ok(otlp_endpoint) = env::var("DSP_META_OTLP_ENDPOINT") {
        eprintln!("Configuring OTLP exporter with endpoint: {}", otlp_endpoint);

        match opentelemetry_otlp::new_exporter()
            .tonic()
            .with_endpoint(otlp_endpoint)
            .build_span_exporter()
        {
            Ok(exporter) => {
                let batch_processor = BatchSpanProcessor::builder(exporter, opentelemetry_sdk::runtime::Tokio).build();
                tracer_provider_builder = tracer_provider_builder.with_span_processor(batch_processor);
                eprintln!("OTLP exporter configured successfully");
            }
            Err(e) => {
                eprintln!("Failed to initialize OTLP exporter: {}", e);
            }
        }
    } else {
        eprintln!("No OTLP endpoint configured (DSP_META_OTLP_ENDPOINT not set). Traces will only be logged locally.");
    }

    let tracer_provider = tracer_provider_builder.build();

    // Set the global tracer provider
    opentelemetry::global::set_tracer_provider(tracer_provider.clone());

    // Set W3C TraceContext propagator as the global propagator
    // This enables extracting trace context from traceparent/tracestate headers
    opentelemetry::global::set_text_map_propagator(
        opentelemetry_sdk::propagation::TraceContextPropagator::new(),
    );

    // Create the OpenTelemetry tracing layer
    let tracer = tracer_provider.tracer("dsp-meta");
    let opentelemetry_layer = tracing_opentelemetry::layer().with_tracer(tracer);

    // configure tracing library at runtime
    match env::var("DSP_META_LOG_FMT") {
        Ok(value) if value.to_lowercase() == "json" => {
            tracing_subscriber::registry()
                .with(opentelemetry_layer)
                .with(fmt::layer().event_format(fmt::format().json()))
                .with(EnvFilter::from_env("DSP_META_LOG_FILTER"))
                .init();
        }
        _ => {
            tracing_subscriber::registry()
                .with(opentelemetry_layer)
                .with(fmt::layer().event_format(fmt::format().compact()))
                .with(EnvFilter::from_env("DSP_META_LOG_FILTER"))
                .init();
        }
    }

    // Run the server
    runtime.block_on(init_server())
}

async fn init_server() {
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    info!("Version: {}", VERSION);

    let settings = Config::builder()
        // Add in settings from the environment (with a prefix of APP)
        // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
        .add_source(config::Environment::with_prefix("DSP_META"))
        .build()
        .unwrap();

    let data_dir = settings
        .get::<String>("data_dir")
        .unwrap_or("/data".to_string());

    let public_dir = settings
        .get::<String>("public_dir")
        .unwrap_or("/public".to_string());

    let base_url = settings
        .get::<Url>("base_url")
        .unwrap_or(Url::parse("https://meta.dasch.swiss").unwrap());

    let shared_state = Arc::new(AppState {
        metadata_service: MetadataService::new(MetadataRepository::from_path(Path::new(&data_dir))),
        public_dir,
        version: VERSION,
        base_url,
    });

    // start the server
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    dbg!("Starting up: http://localhost:3000");
    axum::serve(listener, dsp_meta::api::router::router(shared_state))
        .await
        .unwrap();
}
