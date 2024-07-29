use std::sync::Arc;
use std::time::Duration;

use axum::body::Bytes;
use axum::http::{HeaderMap, Request};
use axum::response::Response;
use axum::routing::get;
use axum::{http, Router};
use log::info;
use tower_http::classify::ServerErrorsFailureClass;
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::{ServeDir, ServeFile};
use tower_http::trace::TraceLayer;
use tracing::{error, info_span, warn, Span};

use crate::api::handler::{health, robots_txt, sitemap_xml, v1};
use crate::app_state::AppState;

/// Having a function that produces our router makes it easy to call it from tests
/// without having to create an HTTP server.
pub fn router(shared_state: Arc<AppState>) -> Router {
    let cors = CorsLayer::new()
        .expose_headers(Any)
        .allow_methods(Any)
        .allow_headers(Any)
        .allow_origin(Any);
    Router::new()
        .route(
            "/api/v1/projects",
            get(v1::projects::handlers::get_by_page_and_filter),
        )
        .route(
            "/api/v1/projects/:shortcode",
            get(v1::projects::handlers::get_by_shortcode),
        )
        .route("/health", get(health::health))
        .route("/version.txt", get(shared_state.version))
        .route("/robots.txt", get(robots_txt::robots_txt))
        .route("/sitemap.xml", get(sitemap_xml::sitemap_xml))
        .fallback_service(
            ServeDir::new(shared_state.public_dir.as_str()).fallback(ServeFile::new(format!(
                "{}/index.html",
                shared_state.public_dir
            ))),
        )
        .with_state(shared_state)
        // See https://docs.rs/tower-http/latest/tower_http/trace/index.html for more details
        // on how to customize.
        .layer(cors)
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
                    span.record("latency", latency.as_millis());
                    if response.status() >= http::StatusCode::INTERNAL_SERVER_ERROR {
                        error!("response handled by server with 5xx");
                    } else if response.status() >= http::StatusCode::BAD_REQUEST {
                        warn!("response handled by server with 4xx")
                    } else {
                        info!("response handled by server");
                    }
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
    use std::env;

    use axum::body::Body;
    use axum::http::StatusCode;
    use http_body_util::BodyExt;
    // for `collect`
    use tower::ServiceExt;
    use url::Url;

    // for `oneshot` and `ready`
    use super::*;
    use crate::domain::service::project_metadata_service::ProjectMetadataService;
    use crate::repo::service::project_metadata_repository::ProjectMetadataRepository;

    #[tokio::test]
    async fn test_health_route() {
        let data_dir = env::current_dir().unwrap().parent().unwrap().join("data");

        let shared_state = Arc::new(AppState {
            project_metadata_service: ProjectMetadataService::new(ProjectMetadataRepository::new(
                &data_dir.as_path(),
            )),
            public_dir: "".to_string(),
            version: "",
            base_url: Url::parse("http://localhost:3000").unwrap(),
        });

        let router = router(shared_state);

        // `Router` implements `tower::Service<Request<Body>>` so we can
        // call it like any tower service, no need to run an HTTP server.
        let response = router
            .oneshot(
                Request::builder()
                    .uri("/health")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        assert_eq!(&body[..], b"healthy");
    }

    #[tokio::test]
    async fn test_not_found_project() {
        let data_dir = env::current_dir().unwrap().parent().unwrap().join("data");

        let shared_state = Arc::new(AppState {
            project_metadata_service: ProjectMetadataService::new(ProjectMetadataRepository::new(
                &data_dir.as_path(),
            )),
            public_dir: "".to_string(),
            version: "",
            base_url: Url::parse("http://localhost:3000").unwrap(),
        });

        let router = router(shared_state);

        // `Router` implements `tower::Service<Request<Body>>` so we can
        // call it like any tower service, no need to run an HTTP server.
        let unknown_shortcode = "9999";
        let response = router
            .oneshot(
                Request::builder()
                    .uri(format!("/api/projects/{}", unknown_shortcode))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }
}
