use axum::http::StatusCode;
use axum::response::{IntoResponse, Json, Response};
use serde::Serialize;

use crate::domain::model::entity::project_metadata::ProjectMetadata;

#[derive(Debug, Default, Clone, PartialEq, Serialize)]
pub struct OptionalProjectMetadata(pub Option<ProjectMetadata>);

impl IntoResponse for ProjectMetadata {
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(serde_json::to_value(self).unwrap())).into_response()
    }
}

impl IntoResponse for OptionalProjectMetadata {
    fn into_response(self) -> Response {
        match self.0 {
            Some(result) => result.into_response(),
            None => (StatusCode::NOT_FOUND).into_response(),
        }
    }
}