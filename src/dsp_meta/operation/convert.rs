use std::fs;
use std::path::PathBuf;

use hcl::Body;
use log::{error, info, trace, warn};

pub fn convert(source_path: &PathBuf, target_path: &PathBuf) -> anyhow::Result<()> {
    let source_string = match fs::read_to_string(source_path) {
        Ok(s) => {
            trace!("Successfully read file at: {}", source_path.display());
            anyhow::Ok(s)
        }
        Err(e) => {
            error!(
                "Could not read the file at the supplied path: {}: {}",
                source_path.display(),
                e.to_string()
            );
            Err(anyhow::anyhow! {e})
        }
    }?;

    let body: Body = serde_json::from_str(source_string.as_str()).unwrap();

    trace!("Body: {:?}", body);

    Ok(())
}
