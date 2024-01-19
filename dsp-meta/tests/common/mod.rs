use std::path::Path;

use dsp_domain::metadata::entity::project_metadata::ProjectMetadata;
use dsp_meta::api::convert::hcl::hcl_body::HclBody;
use dsp_meta::error::DspMetaError;

pub fn load<P: AsRef<Path>>(path: P) -> Result<ProjectMetadata, DspMetaError> {
    let input = std::fs::read_to_string(path)?;
    let body: hcl::Body = hcl::from_str(&input)?;
    let entity: ProjectMetadata = HclBody(&body).try_into()?;
    Ok(entity)
}
