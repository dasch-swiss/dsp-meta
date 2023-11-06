use crate::error::DspMetaError;

impl From<sophia::iri::error::InvalidIri> for DspMetaError {
    fn from(error: sophia::iri::error::InvalidIri) -> Self {
        DspMetaError::SerializeToRdf(error.0)
    }
}

impl From<sophia::term::TermError> for DspMetaError {
    fn from(error: sophia::term::TermError) -> Self {
        DspMetaError::SerializeToRdf(error.to_string())
    }
}
