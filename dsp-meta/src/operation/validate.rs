use std::path::Path;

use dsp_domain::metadata::entity::project_metadata::ProjectMetadata;
use tracing::info;

use crate::api::convert::hcl::hcl_body::HclBody;
use crate::error::DspMetaError;
use crate::infrastructure::load_hcl_file_paths;

/// Read project metadata from .hcl file.
pub fn validate<P: AsRef<Path>>(project_path: &P) -> Result<(), DspMetaError> {
    info!("Hello from validate!");
    let input = std::fs::read_to_string(project_path)?;
    let body: hcl::Body = hcl::from_str(&input)?;
    let _metadata: ProjectMetadata = HclBody(&body).try_into()?;
    Ok(())
}

/// Read project metadata from folder containing .hcl files.
pub fn validate_data(data_path: &Path) -> Result<(), DspMetaError> {
    info!("Entering validate_data()");
    for file in load_hcl_file_paths(data_path) {
        info!("Validating {}", file.display());
        let input = std::fs::read_to_string(file)?;
        let body: hcl::Body = hcl::from_str(&input)?;
        let _entity: ProjectMetadata = HclBody(&body).try_into()?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {}
