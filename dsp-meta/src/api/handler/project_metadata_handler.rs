use std::sync::Arc;

use axum::extract::{Path, State};
use axum::response::Json;
use axum_macros::debug_handler;
use dsp_domain::metadata::entity::project_metadata::ProjectMetadata;
use dsp_domain::metadata::value::Shortcode;
use serde_json::Value;
use tracing::trace;

use crate::api::convert::axum::responses::ProjectMetadataDto;
use crate::api::convert::hcl::hcl_body::HclBody;
use crate::api::convert::rdf::project_metadata::ProjectMetadataGraphDto;
use crate::api::model::project_metadata_dto::ProjectMetadataGraphResult;
use crate::app_state::AppState;
use crate::domain::service::project_metadata_api_contract::ProjectMetadataApiContract;
use crate::error::DspMetaError;

/// GET /project_metadata/:shortcode
/// Get project metadata by shortcode
///
/// TODO: Add error handling with correct status codes
pub async fn get_project_metadata_by_shortcode(
    Path(shortcode): Path<String>,
    State(state): State<Arc<AppState>>,
) -> Result<ProjectMetadataDto, DspMetaError> {
    trace!("entered get_project_metadata_by_shortcode()");
    state
        .project_metadata_service
        .find_by_id(Shortcode(shortcode))
        .map(ProjectMetadataDto)
}

/// GET /project_metadata/:shortcode/rdf
/// Get project metadata by shortcode returned as an RDF string.
pub async fn get_project_metadata_by_shortcode_as_rdf(
    Path(shortcode): Path<String>,
    State(state): State<Arc<AppState>>,
) -> Result<ProjectMetadataGraphResult, DspMetaError> {
    trace!("entered get_project_metadata_by_shortcode_as_rdf()");
    state
        .project_metadata_service
        .find_by_id(Shortcode(shortcode))
        .map(|metadata| metadata.map(|m| ProjectMetadataGraphDto(m).into()))
        .map(ProjectMetadataGraphResult)
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
) -> Result<ProjectMetadataDto, DspMetaError> {
    trace!("entered save_project_metadata");

    let service = &state.project_metadata_service;

    let hcl_body = hcl::from_str(body.as_str())?;
    let project_metadata: ProjectMetadata = HclBody(&hcl_body).try_into()?;

    service
        .save(project_metadata)
        .map(Some)
        .map(ProjectMetadataDto)
}
