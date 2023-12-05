use std::path::Path;

use dsp_domain::metadata::entity::project_metadata::ProjectMetadata;
use tracing::{info, info_span};

use crate::api::convert::hcl::hcl_body::HclBody;
use crate::error::DspMetaError;

/// Read project metadata from .hcl file.
pub fn validate<P: AsRef<Path>>(project_path: &P) -> Result<(), DspMetaError> {
    info!("Hello from validate!");
    let input = std::fs::read_to_string(project_path)?;
    let body: hcl::Body = hcl::from_str(&input)?;
    let _metadata: ProjectMetadata = HclBody(&body).try_into()?;
    Ok(())
}

/// Read project metadata from folder containing .hcl files.
pub fn validate_data<P: AsRef<Path>>(data_path: &P) -> Result<(), DspMetaError> {
    info!("Entering validate_data()");

    // get paths of HCL files
    let hcl_files = std::fs::read_dir(data_path)
        .expect("read directory containing HCL files.")
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, std::io::Error>>()
        .expect("collect all files into collection.");

    // load into db
    for file in hcl_files {
        info!("Validating {}", file.display());
        let input = std::fs::read_to_string(file)?;
        let body: hcl::Body = hcl::from_str(&input)?;
        let _entity: ProjectMetadata = HclBody(&body).try_into()?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {}
