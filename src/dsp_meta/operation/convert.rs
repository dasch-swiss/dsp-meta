use std::path::Path;

use tracing::info;

use crate::domain::entity::project_metadata::ProjectMetadata;
use crate::errors::DspMetaError;

pub fn convert<P: AsRef<Path>>(source_path: &P, _target_path: &P) -> Result<(), DspMetaError> {
    info!("Hello from convert!");
    let input = std::fs::read_to_string(source_path)?;
    let body: hcl::Body = hcl::from_str(&input)?;
    let _metadata = ProjectMetadata::try_from(&body)?;

    Ok(())
}
