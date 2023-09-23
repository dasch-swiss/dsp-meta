use std::sync::Arc;

use axum::routing::{get, post};
use axum::Router;
use dsp_meta::api;
use dsp_meta::repo::project_metadata_repository::ProjectMetadataRepository;
use dsp_meta::service::project_metadata_service::ProjectMetadataService;
use tracing::{trace, Level};
use tracing_subscriber::FmtSubscriber;

struct AppState {
    project_metadata_service: ProjectMetadataService<ProjectMetadataRepository>,
}

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
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        // `GET /` goes to `root`
        .route("/", get(api::root))
        .route("/foo", get(api::get_foo).post(api::post_foo))
        .route("/foo/bar", get(api::foo_bar))
        // `POST /users` goes to `create_user`
        .route("/users", post(api::create_user))
        .with_state(shared_state);

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
