use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::Serialize;

use crate::domain::model::project_info::ProjectInfo;

#[derive(Debug, Default, Clone, PartialEq, Serialize)]
pub struct ProjectInfosDto(pub Vec<ProjectInfo>);

/// Convert `ProjectInfosDto` into a response.
impl IntoResponse for ProjectInfosDto {
    fn into_response(self) -> Response {
        (
            StatusCode::OK,
            Json(serde_json::to_value(self.0).expect("project infos should convert to JSON.")),
        )
            .into_response()
    }
}
