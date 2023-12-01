use dsp_domain::metadata::entity::project_metadata::ProjectMetadata;
use serde::Serialize;

use crate::api::convert::rdf::project_metadata::ProjectMetadataGraph;

#[derive(Debug, Default, Clone, PartialEq, Serialize)]
pub struct ProjectMetadataDto(pub Option<ProjectMetadata>);

pub struct ProjectMetadataGraphDto(pub Option<ProjectMetadataGraph>);
