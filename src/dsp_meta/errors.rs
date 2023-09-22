use std::io;

#[derive(Debug)]
pub enum DspMetaError {
    IO(io::Error),
    ParseHcl(hcl::Error),
    UnknownAttribute(String),
    ParseVersion(String),
    ParseProject(String),
    ParseDataset(String),
    CreateValueObject(String),
    NotFound,
}

impl From<io::Error> for DspMetaError {
    fn from(error: io::Error) -> Self {
        DspMetaError::IO(error)
    }
}

impl From<hcl::Error> for DspMetaError {
    fn from(error: hcl::Error) -> Self {
        DspMetaError::ParseHcl(error)
    }
}
