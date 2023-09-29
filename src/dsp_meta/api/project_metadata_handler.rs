use std::sync::Arc;

use axum::extract::{Path, State};
use axum::response::Json;
use axum_macros::debug_handler;
use serde_json::Value;
use tracing::trace;

use crate::app_state::AppState;
use crate::domain::model::entity::project_metadata::ProjectMetadata;
use crate::domain::model::value::Shortcode;
use crate::domain::service::project_metadata_api_contract::ProjectMetadataApiContract;
use crate::errors::DspMetaError;

/// GET /project_metadata/:shortcode
/// Get project metadata by shortcode
///
/// TODO: Add error handling with correct status codes
pub async fn get_project_metadata_by_shortcode(
    Path(shortcode): Path<String>,
    State(state): State<Arc<AppState>>,
) -> Json<Value> {
    trace!("entered get_project_metadata_by_shortcode()");
    let project_metadata = state
        .project_metadata_service
        .find_by_id(Shortcode(shortcode));
    Json(serde_json::to_value(project_metadata).unwrap())
}

pub async fn get_all_project_metadata(State(state): State<Arc<AppState>>) -> Json<Value> {
    trace!("entered get_all_project_metadata()");
    let all_project_metadata = state.project_metadata_service.find_all();
    Json(serde_json::to_value(all_project_metadata).unwrap())
}

#[debug_handler]
pub async fn save_project_metadata(
    State(state): State<Arc<AppState>>,
    body: String,
) -> Result<ProjectMetadata, DspMetaError> {
    trace!("entered save_project_metadata");

    let service = &state.project_metadata_service;

    let hcl_body = hcl::from_str(body.as_str())?;
    let project_metadata = ProjectMetadata::try_from(&hcl_body)?;

    service.save(project_metadata)
}
