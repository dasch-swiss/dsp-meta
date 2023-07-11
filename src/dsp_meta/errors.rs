use std::io;

#[derive(Debug)]
pub enum DspMetaError {
    IO(io::Error),
    ParseHcl(hcl::Error),
    ParseToml(toml::de::Error),
    ParseVersion(&'static str),
    ParseProject(&'static str),
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

impl From<toml::de::Error> for DspMetaError {
    fn from(value: toml::de::Error) -> Self {
        DspMetaError::ParseToml(value)
    }
}
