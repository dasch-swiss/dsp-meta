use axum::http::StatusCode;
use axum::response::{IntoResponse, Json, Response};

use crate::domain::model::entity::project_metadata::ProjectMetadata;

impl IntoResponse for ProjectMetadata {
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(serde_json::to_value(self).unwrap())).into_response()
    }
}
