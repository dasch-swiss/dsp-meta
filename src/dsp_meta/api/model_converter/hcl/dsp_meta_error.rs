use crate::errors::DspMetaError;

impl From<hcl::Error> for DspMetaError {
    fn from(error: hcl::Error) -> Self {
        DspMetaError::ParseHcl(error.to_string())
    }
}
