use std::path::Path;

use tracing::info;

use crate::errors::DspMetaError;
use crate::load;

pub fn convert<P: AsRef<Path>>(source_path: &P, _target_path: &P) -> Result<(), DspMetaError> {
    info!("Hello from convert!");
    let _ = load(source_path)?;

    Ok(())
}
