use std::path::Path;

use tracing::info;

use crate::errors::DspMetaError;
use crate::load;

/// Read projects definition from .toml
pub fn validate<P: AsRef<Path>>(project_path: &P) -> Result<(), DspMetaError> {
    info!("Hello from validate!");
    let _ = load(project_path)?;
    Ok(())
}

#[cfg(test)]
mod tests {}
