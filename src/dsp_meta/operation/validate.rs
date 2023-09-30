use std::path::Path;

use tracing::info;

use crate::domain::model::entity::project_metadata::ProjectMetadata;
use crate::errors::DspMetaError;

/// Read projects definition from .toml
pub fn validate<P: AsRef<Path>>(project_path: &P) -> Result<(), DspMetaError> {
    info!("Hello from validate!");
    let input = std::fs::read_to_string(project_path)?;
    let body: hcl::Body = hcl::from_str(&input)?;
    let _metadata = ProjectMetadata::try_from(&body)?;
    Ok(())
}

#[cfg(test)]
mod tests {}
