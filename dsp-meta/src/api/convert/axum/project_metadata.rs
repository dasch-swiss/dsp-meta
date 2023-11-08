use axum::http::StatusCode;
use axum::response::{IntoResponse, Json, Response};
use dsp_domain::metadata::entity::project_metadata::ProjectMetadata;
use serde::Serialize;

#[derive(Debug, Default, Clone, PartialEq, Serialize)]
pub struct ProjectMetadataDto(pub Option<ProjectMetadata>);

impl IntoResponse for ProjectMetadataDto {
    fn into_response(self) -> Response {
        match self.0 {
            Some(pm) => (StatusCode::OK, Json(serde_json::to_value(pm).unwrap())).into_response(),
            None => (StatusCode::NOT_FOUND).into_response(),
        }
    }
}
