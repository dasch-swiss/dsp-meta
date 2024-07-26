use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::Serialize;

use crate::domain::model::draft_model::{DraftMetadata, DraftProjectStatus};

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ProjectMetadataDto(pub DraftMetadata);

/// Convert `ProjectMetadataDto` into an Axum response.
impl IntoResponse for ProjectMetadataDto {
    fn into_response(self) -> Response {
        let metadata = self.0;
        (
            StatusCode::OK,
            Json(serde_json::to_value(metadata).expect("project metadata should convert to JSON.")),
        )
            .into_response()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ProjectMetadataWithInfoDto {
    id: String,
    name: String,
    description: String,
    status: DraftProjectStatus,
    metadata: ProjectMetadataDto,
}

impl From<DraftMetadata> for ProjectMetadataWithInfoDto {
    fn from(value: DraftMetadata) -> Self {
        let project = value.project.clone();
        ProjectMetadataWithInfoDto {
            id: project.shortcode,
            name: project.name,
            description: project.teaser_text,
            status: project.status.unwrap_or_default(),
            metadata: ProjectMetadataDto(value.clone()),
        }
    }
}
