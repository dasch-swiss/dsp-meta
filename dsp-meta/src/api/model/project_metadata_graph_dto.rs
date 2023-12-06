use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

use crate::api::convert::rdf::project_metadata::ProjectMetadataGraph;

pub struct ProjectMetadataGraphDto(pub Option<ProjectMetadataGraph>);

/// Convert `ProjectMetadataGraph` into a response.
impl IntoResponse for ProjectMetadataGraphDto {
    fn into_response(self) -> Response {
        match self.0 {
            Some(metadata_graph) => {
                (StatusCode::OK, metadata_graph.to_turtle_string()).into_response()
            }
            None => StatusCode::NOT_FOUND.into_response(),
        }
    }
}
