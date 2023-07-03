use std::fs;
use std::path::PathBuf;

use log::{error, info, trace, warn};

use crate::domain::project::Project;

/// Read projects definition from .toml
pub fn validate(project_path: &PathBuf) -> anyhow::Result<()> {
    info!("Hello from validate!");

    let toml = match fs::read_to_string(project_path) {
        Ok(s) => {
            trace!("Successfully read file at: {}", project_path.display());
            anyhow::Ok(s)
        }
        Err(e) => {
            error!(
                "Could not read the file at the supplied path: {}",
                e.to_string()
            );
            Err(anyhow::anyhow!(e))
        }
    }?;
    trace!("read project: \n{toml}");
    let _: Project = toml::from_str(toml.as_str())?;
    anyhow::Ok(())
}

#[cfg(test)]
mod tests {}
