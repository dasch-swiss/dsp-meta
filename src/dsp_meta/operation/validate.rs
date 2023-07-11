use std::fs;
use std::path::Path;

use log::{error, info, trace};

use crate::domain::Project;
use crate::errors::DspMetaError;

/// Read projects definition from .toml
pub fn validate<P: AsRef<Path>>(project_path: &P) -> Result<(), DspMetaError> {
    info!("Hello from validate!");

    let toml = match fs::read_to_string(project_path) {
        Ok(s) => {
            trace!(
                "Successfully read file at: {}",
                project_path.as_ref().display()
            );
            Ok(s)
        }
        Err(e) => {
            error!(
                "Could not read the file at the supplied path: {}",
                e.to_string()
            );
            Err(e)
        }
    }?;
    trace!("read project: \n{toml}");
    let _: Project = toml::from_str(toml.as_str())?;
    Ok(())
}

#[cfg(test)]
mod tests {}
