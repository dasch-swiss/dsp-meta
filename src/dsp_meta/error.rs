use std::io;

use serde::Serialize;
use thiserror::Error;

/// Type alias for `Result` with default error `InvalidIri`.
///
/// Can be used like `std::result::Result` as well.
pub type Result<T, E = DspMetaError> = std::result::Result<T, E>;

/// This error is raised when trying to parse an invalid IRI.
#[derive(Debug, Error, Serialize)]
pub enum DspMetaError {
    #[error("IO error: `{0}`")]
    IO(String),
    #[error("Error parsing the HCL input: `{0}`")]
    ParseHcl(String),
    #[error("The provided attribute is unknown: `{0}`")]
    UnknownAttribute(String),
    #[error("Error parsing the version: `{0}`")]
    ParseVersion(String),
    #[error("Error parsing the project: `{0}`")]
    ParseProject(String),
    #[error("Error parsing the dataset: `{0}`")]
    ParseDataset(String),
    #[error("Error creating value object: `{0}`")]
    CreateValueObject(String),
    #[error("Error serializing to RDF: `{0}`")]
    SerializeToRdf(String),
    #[error("The requested resource was not found")]
    NotFound,
}

impl From<io::Error> for DspMetaError {
    fn from(error: io::Error) -> Self {
        DspMetaError::IO(error.to_string())
    }
}
