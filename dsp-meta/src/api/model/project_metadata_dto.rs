use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use dsp_domain::metadata::entity::project_metadata::ProjectMetadata;
use serde::Serialize;

#[derive(Debug, Default, Clone, PartialEq, Serialize)]
pub struct ProjectMetadataDto(pub Option<ProjectMetadata>);

/// Convert `ProjectMetadataDto` into an Axum response.
impl IntoResponse for ProjectMetadataDto {
    fn into_response(self) -> Response {
        match self.0 {
            Some(metadata) => (
                StatusCode::OK,
                Json(
                    serde_json::to_value(metadata)
                        .expect("project metadata should convert to JSON."),
                ),
            )
                .into_response(),
            None => (StatusCode::NOT_FOUND).into_response(),
        }
    }
}
