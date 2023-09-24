use std::sync::Arc;

use axum::routing::{get, post};
use axum::Router;
use dsp_meta::api;
use dsp_meta::repo::project_metadata_repository::ProjectMetadataRepository;
use dsp_meta::service::project_metadata_service::ProjectMetadataService;
use tracing::{trace, Level};
use tracing_subscriber::FmtSubscriber;

#[derive(Clone)]
pub struct AppState {
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

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app(shared_state).into_make_service())
        .await
        .unwrap();
}

/// Having a function that produces our app makes it easy to call it from tests
/// without having to create an HTTP server.
fn app(shared_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", get(api::get_root).post(api::post_root))
        .route("/hello_world", get(api::hello_world))
        .route("/foo/bar", get(api::foo_bar))
        // `POST /users` goes to `create_user`
        .route("/users", post(api::create_user))
        .with_state(shared_state)
}

#[cfg(test)]
mod tests {
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    // use tower::Service; // for `call`
    use tower::ServiceExt; // for `oneshot` and `ready`

    use super::*;

    #[tokio::test]
    async fn hello_world() {
        let shared_state = Arc::new(AppState {
            project_metadata_service: ProjectMetadataService::new(ProjectMetadataRepository::new()),
        });

        let app = app(shared_state);

        // `Router` implements `tower::Service<Request<Body>>` so we can
        // call it like any tower service, no need to run an HTTP server.
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/hello_world")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        assert_eq!(&body[..], b"Hello, World!");
    }
}
