use std::io;

#[derive(Debug)]
pub enum DspMetaError {
    IO(io::Error),
    ParseHcl(hcl::Error),
    UnknownAttribute(&'static str),
    ParseVersion(&'static str),
    ParseProject(&'static str),
    ParseDataset(&'static str),
    CreateValueObject(&'static str),
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
