use std::sync::Arc;

use dsp_meta::app_state::AppState;
use dsp_meta::domain::service::project_metadata_service::ProjectMetadataService;
use dsp_meta::repo::service::project_metadata_repository::ProjectMetadataRepository;
use tracing::trace;
use tracing_subscriber::layer::SubscriberExt;

#[tokio::main]
async fn main() {
    // configure tracing library
    let stdout_log = tracing_subscriber::fmt::layer();

    let subscriber = tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                // axum logs rejections from built-in extractors with the `axum::rejection`
                // target, at `TRACE` level. `axum::rejection=trace` enables showing those events
                "dsp_meta=trace,tower_http=trace,axum::rejection=trace".into()
            }),
        )
        .with(stdout_log);

    // Attempts to set as the global default subscriber.
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    trace!("Hello, world!");

    let shared_state = Arc::new(AppState {
        project_metadata_service: ProjectMetadataService::new(ProjectMetadataRepository::new()),
    });

    // build our application with a single route

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(dsp_meta::api::app::app(shared_state).into_make_service())
        .await
        .unwrap();
}
