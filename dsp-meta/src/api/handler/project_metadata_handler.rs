use std::sync::Arc;

use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use dsp_domain::metadata::value::Shortcode;
use tracing::{info_span, instrument, trace};

use crate::api::convert::rdf::project_metadata::ProjectMetadataGraphWrapper;
use crate::api::model::project_metadata_dto::{ProjectMetadataDto, ProjectMetadataWithInfoDto};
use crate::api::model::project_metadata_graph_dto::ProjectMetadataGraphDto;
use crate::app_state::AppState;
use crate::domain::service::project_metadata_api_contract::ProjectMetadataApiContract;
use crate::domain::service::repository_contract::Pagination;
use crate::error::DspMetaError;

/// GET /project_metadata/:shortcode
/// Get project metadata by shortcode
///
/// TODO: Add error handling with correct status codes
#[instrument(skip(state))]
pub async fn get_project_metadata_by_shortcode(
    Path(shortcode): Path<String>,
    State(state): State<Arc<AppState>>,
) -> Result<Response, DspMetaError> {
    trace!("entered get_project_metadata_by_shortcode()");
    state
        .project_metadata_service
        .find_by_id(Shortcode(shortcode))
        .map(|option| match option {
            Some(metadata) => (StatusCode::OK, ProjectMetadataDto(metadata)).into_response(),
            None => (StatusCode::NOT_FOUND, "No project 9999 available").into_response(),
        })
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
    pagination: Option<Query<Pagination>>,
) -> Result<axum::Json<Vec<ProjectMetadataWithInfoDto>>, DspMetaError> {
    trace!("entered get_all_project_metadata()");
    let Query(pagination) = pagination.unwrap_or_default();
    state
        .project_metadata_service
        .find_all(&pagination)
        .map(|metadata| {
            metadata
                .into_iter()
                .map(ProjectMetadataWithInfoDto::from)
                .collect::<Vec<ProjectMetadataWithInfoDto>>()
        })
        .map(axum::Json)
}
