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
use opentelemetry_sdk::trace::TracerProvider;
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

    // Initialize OpenTelemetry tracer provider
    // Note: In opentelemetry_sdk 0.26, resources are configured via config()
    let resource = Resource::new(vec![KeyValue::new("service.name", "dsp-meta")]);
    let tracer_provider = TracerProvider::builder()
        .with_config(opentelemetry_sdk::trace::Config::default().with_resource(resource))
        .build();

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

    // Manually create a tokio runtime (as opposed to using the macro)
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(init_server())
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
