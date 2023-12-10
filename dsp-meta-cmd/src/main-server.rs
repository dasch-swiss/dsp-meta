use std::env;
use std::sync::Arc;
use std::time::Duration;

use config::Config;
use dsp_meta::app_state::AppState;
use dsp_meta::domain::service::project_metadata_service::ProjectMetadataService;
use dsp_meta::repo::service::project_metadata_repository::ProjectMetadataRepository;
use opentelemetry::trace::TracerProvider as _;
use opentelemetry::{global, KeyValue};
use opentelemetry_sdk::metrics::{MeterProvider, PeriodicReader};
use opentelemetry_sdk::runtime;
use opentelemetry_sdk::trace::TracerProvider;
use opentelemetry_stdout as stdout;
use pid1::Pid1Settings;
use tokio::net::TcpListener;
use tracing::{info, span};
use tracing_opentelemetry::MetricsLayer;
use tracing_subscriber::prelude::*;
use tracing_subscriber::{fmt, EnvFilter};

fn main() {
    // Do the pid1 magic. Needs to be the first thing executed.
    Pid1Settings::new()
        .enable_log(true)
        .timeout(Duration::from_secs(2))
        .launch()
        .expect("pid1 launch");

    // Manually create a tokio runtime (as opposed to using the macro)
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(init_server())
}

async fn init_server() {
    init_tracing_subscriber();

    info!("Ivan was here!");

    const VERSION: &str = env!("CARGO_PKG_VERSION");
    info!("Version: {}", VERSION);

    // get a meter from a provider
    let meter = global::meter("dsp-meta-server");

    // create an instrument
    let counter = meter.u64_counter("dsp_meta_server_info").init();

    // record a measurement
    counter.add(1, &[KeyValue::new("version", VERSION)]);

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

    let shared_state = Arc::new(AppState {
        project_metadata_service: ProjectMetadataService::new(ProjectMetadataRepository::new(
            &data_dir,
        )),
        public_dir,
        version: VERSION,
    });

    let root = span!(tracing::Level::TRACE, "app_start");
    let _enter = root.enter();

    // start the server
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, dsp_meta::api::router::router(shared_state))
        .await
        .unwrap();
}

/// Configure tracing library at runtime
fn init_tracing_subscriber() {
    match env::var("DSP_META_LOG_FMT") {
        Ok(value) if value.to_lowercase() == "json" => tracing_subscriber::registry()
            .with(fmt::layer().event_format(fmt::format().json()))
            .with(EnvFilter::from_env("DSP_META_LOG_FILTER"))
            .with(
                tracing_opentelemetry::layer().with_tracer(init_tracer().tracer("dsp-meta-server")),
            )
            .with(MetricsLayer::new(init_meter()))
            .init(),
        _ => tracing_subscriber::registry()
            .with(fmt::layer().event_format(fmt::format().compact()))
            .with(EnvFilter::from_env("DSP_META_LOG_FILTER"))
            .with(
                tracing_opentelemetry::layer().with_tracer(init_tracer().tracer("dsp-meta-server")),
            )
            .with(MetricsLayer::new(init_meter()))
            .init(),
    }
}

/// Initialize a new OpenTelemetry trace pipeline that prints to stdout
fn init_tracer() -> TracerProvider {
    let exporter = stdout::SpanExporter::default();
    TracerProvider::builder()
        .with_simple_exporter(exporter)
        .build()
}

/// Initialize a new OpenTelemetry metrics pipeline that prints to stdout
fn init_meter() -> MeterProvider {
    let exporter = opentelemetry_stdout::MetricsExporter::default();
    let reader = PeriodicReader::builder(exporter, runtime::Tokio).build();
    MeterProvider::builder().with_reader(reader).build()
}
