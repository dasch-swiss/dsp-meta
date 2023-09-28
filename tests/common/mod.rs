use std::path::Path;

use dsp_meta::domain::model::entity::project_metadata::ProjectMetadata;
use dsp_meta::errors::DspMetaError;

pub fn load<P: AsRef<Path>>(path: P) -> Result<ProjectMetadata, DspMetaError> {
    let input = std::fs::read_to_string(path)?;
    let body: hcl::Body = hcl::from_str(&input)?;
    let metadata = ProjectMetadata::try_from(&body)?;
    Ok(metadata)
}
