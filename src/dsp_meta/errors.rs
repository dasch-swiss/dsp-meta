use std::io;

use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum DspMetaError {
    IO(String),
    ParseHcl(String),
    UnknownAttribute(String),
    ParseVersion(String),
    ParseProject(String),
    ParseDataset(String),
    CreateValueObject(String),
    SerializeToTtl(String),
    NotFound,
}

impl From<io::Error> for DspMetaError {
    fn from(error: io::Error) -> Self {
        DspMetaError::IO(error.to_string())
    }
}
