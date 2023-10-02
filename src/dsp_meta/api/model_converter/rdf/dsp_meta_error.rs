use crate::errors::DspMetaError;
impl From<sophia::iri::error::InvalidIri> for DspMetaError {
    fn from(error: sophia::iri::error::InvalidIri) -> Self {
        DspMetaError::ParseHcl(error.0)
    }
}
