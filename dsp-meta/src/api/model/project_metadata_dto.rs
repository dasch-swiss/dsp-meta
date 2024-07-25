use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use dsp_domain::metadata::entity::project_metadata::ProjectMetadata;
use dsp_domain::metadata::value::status::Status;
use dsp_domain::metadata::value::{Name, Shortcode, TeaserText};
use serde::Serialize;

#[derive(Debug, Default, Clone, PartialEq, Serialize)]
pub struct ProjectMetadataDto(pub ProjectMetadata);

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

#[derive(Debug, Default, Clone, PartialEq, Serialize)]
pub struct ProjectMetadataWithInfoDto {
    id: Shortcode,
    name: Name,
    description: TeaserText,
    status: Status,
    metadata: ProjectMetadataDto,
}

impl From<ProjectMetadata> for ProjectMetadataWithInfoDto {
    fn from(value: ProjectMetadata) -> Self {
        let project = value.project.clone();
        ProjectMetadataWithInfoDto {
            id: project.shortcode,
            name: project.name,
            description: project.teaser_text,
            status: project.status,
            metadata: ProjectMetadataDto(value.clone()),
        }
    }
}
