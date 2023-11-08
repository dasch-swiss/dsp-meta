use std::sync::Arc;
use std::time::Duration;

use axum::body::Bytes;
use axum::http::{HeaderMap, Request};
use axum::response::Response;
use axum::routing::get;
use axum::Router;
use tower_http::classify::ServerErrorsFailureClass;
use tower_http::trace::TraceLayer;
use tracing::{info_span, trace, Span};

use crate::api::handler::{health, project_metadata_handler};
use crate::app_state::AppState;

/// Having a function that produces our app makes it easy to call it from tests
/// without having to create an HTTP server.
pub fn app(shared_state: Arc<AppState>) -> Router {
    Router::new()
        .route(
            "/projects",
            get(project_metadata_handler::get_all_project_metadata)
                .post(project_metadata_handler::save_project_metadata),
        )
        .route(
            "/projects/:shortcode",
            get(project_metadata_handler::get_project_metadata_by_shortcode),
        )
        .route("/health", get(health::health_handler))
        .with_state(shared_state)
        // See https://docs.rs/tower-http/latest/tower_http/trace/index.html for more details
        // on how to customize.
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|request: &Request<_>| {
                    info_span!(
                        "http_request",
                        method = ?request.method(),
                        uri = request.uri().to_string(),
                        status_code = tracing::field::Empty,
                        latency = tracing::field::Empty,
                    )
                })
                .on_request(|_request: &Request<_>, _span: &Span| ())
                .on_response(|response: &Response, latency: Duration, span: &Span| {
                    span.record("status_code", response.status().as_u16());
                    span.record("latency", latency.as_micros());
                    trace!("response handled by server");
                })
                .on_body_chunk(|_chunk: &Bytes, _latency: Duration, _span: &Span| ())
                .on_eos(
                    |_trailers: Option<&HeaderMap>, _stream_duration: Duration, _span: &Span| (),
                )
                .on_failure(
                    |_error: ServerErrorsFailureClass, _latency: Duration, _span: &Span| (),
                ),
        )
}

#[cfg(test)]
mod tests {
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    // use tower::Service; // for `call`
    use tower::ServiceExt; // for `oneshot` and `ready`

    use super::*;
    use crate::domain::service::project_metadata_service::ProjectMetadataService;
    use crate::repo::service::project_metadata_repository::ProjectMetadataRepository;

    #[tokio::test]
    async fn test_health_route() {
        let shared_state = Arc::new(AppState {
            project_metadata_service: ProjectMetadataService::new(ProjectMetadataRepository::new()),
        });

        let app = app(shared_state);

        // `Router` implements `tower::Service<Request<Body>>` so we can
        // call it like any tower service, no need to run an HTTP server.
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/health")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        assert_eq!(&body[..], b"healthy");
    }

    #[tokio::test]
    async fn test_not_found_project() {
        let shared_state = Arc::new(AppState {
            project_metadata_service: ProjectMetadataService::new(ProjectMetadataRepository::new()),
        });

        let app = app(shared_state);

        // `Router` implements `tower::Service<Request<Body>>` so we can
        // call it like any tower service, no need to run an HTTP server.
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/projects/nonexistent_shortcode")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }
}