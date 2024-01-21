use std::io;

use dsp_domain::error::DspDomainError;
use serde::Serialize;
use serde_json::Error;
use thiserror::Error;

/// Type alias for `Result` with default error `DspMetaError`.
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
    #[error("Error creating domain object: `{0}`")]
    CreateDomainObject(String),
    #[error("The requested resource was not found")]
    NotFound,
    #[error("Error serializing to Json")]
    JsonSerialization(String),
}

impl From<io::Error> for DspMetaError {
    fn from(error: io::Error) -> Self {
        DspMetaError::IO(error.to_string())
    }
}

impl From<DspDomainError> for DspMetaError {
    fn from(value: DspDomainError) -> Self {
        match value {
            DspDomainError::CreateValueObject(err) => DspMetaError::CreateValueObject(err),
            DspDomainError::CreateDomainObject(err) => DspMetaError::CreateDomainObject(err),
        }
    }
}

impl From<serde_json::Error> for DspMetaError {
    fn from(value: Error) -> Self {
        DspMetaError::JsonSerialization(value.to_string())
    }
}
