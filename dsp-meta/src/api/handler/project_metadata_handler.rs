use std::sync::Arc;

use axum::extract::{Path, State};
use dsp_domain::metadata::value::Shortcode;
use tracing::{info_span, instrument, trace};

use crate::api::convert::rdf::project_metadata::ProjectMetadataGraphWrapper;
use crate::api::model::project_infos_dto::ProjectInfosDto;
use crate::api::model::project_metadata_dto::ProjectMetadataDto;
use crate::api::model::project_metadata_graph_dto::ProjectMetadataGraphDto;
use crate::app_state::AppState;
use crate::domain::service::project_metadata_api_contract::ProjectMetadataApiContract;
use crate::error::DspMetaError;

/// GET /project_metadata/:shortcode
/// Get project metadata by shortcode
///
/// TODO: Add error handling with correct status codes
#[instrument(skip(state))]
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

#[instrument(skip(state))]
pub async fn get_project_metadata_by_shortcode_as_rdf(
    Path(shortcode): Path<String>,
    State(state): State<Arc<AppState>>,
) -> Result<ProjectMetadataGraphDto, DspMetaError> {
    info_span!("get project by shortcode as RDF");
    trace!("entered get_project_metadata_by_shortcode_as_rdf()");
    state
        .project_metadata_service
        .find_by_id(Shortcode(shortcode))
        .map(|metadata| metadata.map(|m| ProjectMetadataGraphWrapper(m).into()))
        .map(ProjectMetadataGraphDto)
}

#[instrument(skip(state))]
pub async fn get_all_project_metadata(
    State(state): State<Arc<AppState>>,
) -> Result<ProjectInfosDto, DspMetaError> {
    trace!("entered get_all_project_metadata()");
    state
        .project_metadata_service
        .find_all()
        .map(ProjectInfosDto)
}

#[instrument(skip(state))]
pub async fn get_projects_count(State(state): State<Arc<AppState>>) -> String {
    trace!("entered get_projects_count()");
    let count = state.project_metadata_service.count();
    count.unwrap().to_string()
}
