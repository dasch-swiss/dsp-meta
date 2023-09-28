use std::sync::Arc;

use dsp_meta::app_state::AppState;
use dsp_meta::domain::service::project_metadata_service::ProjectMetadataService;
use dsp_meta::repo::project_metadata_repository::ProjectMetadataRepository;
use tracing::{trace, Level};
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() {
    // configure tracing library
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();

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
