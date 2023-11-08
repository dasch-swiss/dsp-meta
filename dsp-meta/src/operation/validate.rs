use std::path::Path;

use dsp_domain::metadata::entity::project_metadata::ProjectMetadata;
use tracing::info;

use crate::api::convert::hcl::hcl_body::HclBody;
use crate::error::DspMetaError;

/// Read projects definition from .toml
pub fn validate<P: AsRef<Path>>(project_path: &P) -> Result<(), DspMetaError> {
    info!("Hello from validate!");
    let input = std::fs::read_to_string(project_path)?;
    let body: hcl::Body = hcl::from_str(&input)?;
    let _metadata: ProjectMetadata = HclBody(&body).try_into()?;
    Ok(())
}

#[cfg(test)]
mod tests {}