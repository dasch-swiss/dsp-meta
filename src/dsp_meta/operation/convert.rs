use std::fs;
use std::path::Path;

use hcl::Body;
use log::{error, trace};

use crate::errors::DspMetaError;

pub fn convert<P: AsRef<Path>>(source_path: &P, _target_path: &P) -> Result<(), DspMetaError> {
    let source_string = match fs::read_to_string(source_path) {
        Ok(s) => {
            trace!(
                "Successfully read file at: {}",
                &source_path.as_ref().display()
            );
            Ok(s)
        }
        Err(e) => {
            error!(
                "Could not read the file at the supplied path: {}: {}",
                source_path.as_ref().display(),
                e.to_string()
            );
            Err(e)
        }
    }?;

    let body: Body = serde_json::from_str(source_string.as_str()).unwrap();

    trace!("Body: {:?}", body);

    Ok(())
}
