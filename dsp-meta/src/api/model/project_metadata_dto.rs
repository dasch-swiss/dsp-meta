use dsp_domain::metadata::entity::project_metadata::ProjectMetadata;
use serde::Serialize;

#[derive(Debug, Default, Clone, PartialEq, Serialize)]
pub struct ProjectMetadataDto(pub Option<ProjectMetadata>);
