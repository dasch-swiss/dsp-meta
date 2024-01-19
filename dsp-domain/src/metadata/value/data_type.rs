use serde::Serialize;

use crate::error::DspDomainError;

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
pub enum DataType {
    #[default]
    Text,
    Image,
    Audio,
    Video,
}

impl TryFrom<String> for DataType {
    type Error = DspDomainError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "Text" => Ok(DataType::Text),
            "Image" => Ok(DataType::Image),
            "Audio" => Ok(DataType::Audio),
            "Video" => Ok(DataType::Video),
            _ => Err(DspDomainError::CreateValueObject(format!(
                "Creating Status failed because provided value '{}' is not allowed.",
                value
            ))),
        }
    }
}
