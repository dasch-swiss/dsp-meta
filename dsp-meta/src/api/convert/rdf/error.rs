use crate::error::DspMetaError;

impl From<sophia::iri::InvalidIri> for DspMetaError {
    fn from(error: sophia::iri::InvalidIri) -> Self {
        DspMetaError::SerializeToRdf(error.0)
    }
}
