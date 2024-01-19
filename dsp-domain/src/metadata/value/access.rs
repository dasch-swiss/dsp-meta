use serde::Serialize;

use crate::error::DspDomainError;

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
pub enum Access {
    #[default]
    Open,
    Restricted,
    Closed,
}

impl TryFrom<String> for Access {
    type Error = DspDomainError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "Open" => Ok(Access::Open),
            "Restricted" => Ok(Access::Restricted),
            "Closed" => Ok(Access::Closed),
            _ => Err(DspDomainError::CreateValueObject(format!(
                "Creating Access failed because provided value '{}' is not allowed.",
                value
            ))),
        }
    }
}
