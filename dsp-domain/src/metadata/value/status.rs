use serde::Serialize;

use crate::error::DspDomainError;

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
pub enum Status {
    #[default]
    Ongoing,
    Finished,
}

impl TryFrom<String> for Status {
    type Error = DspDomainError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "Ongoing" => Ok(Status::Ongoing),
            "Finished" => Ok(Status::Finished),
            _ => Err(DspDomainError::CreateValueObject(format!(
                "Creating an Status failed because provided value '{}' is not allowed.",
                value
            ))),
        }
    }
}
