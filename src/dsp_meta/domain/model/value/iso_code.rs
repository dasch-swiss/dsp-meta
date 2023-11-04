use std::fmt::{Display, Formatter};

use serde::Serialize;

use crate::error::DspMetaError;

/// Language codes according to ISO 639-1
/// Not an exhaustive list.
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Serialize)]
#[serde(rename_all(serialize = "lowercase"))]
pub enum IsoCode {
    #[default]
    DE, // German
    EN, // English
    FR, // French
    IT, // Italian
    ES, // Spanish
    PT, // Portuguese
    NL, // Dutch
    PL, // Polish
    RU, // Russian
    JA, // Japanese
    ZH, // Chinese
    AR, // Arabic
    FA, // Persian
}

impl Display for IsoCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            IsoCode::DE => write!(f, "de"),
            IsoCode::EN => write!(f, "en"),
            IsoCode::FR => write!(f, "fr"),
            IsoCode::IT => write!(f, "it"),
            IsoCode::ES => write!(f, "es"),
            IsoCode::PT => write!(f, "pt"),
            IsoCode::NL => write!(f, "nl"),
            IsoCode::PL => write!(f, "pl"),
            IsoCode::RU => write!(f, "ru"),
            IsoCode::JA => write!(f, "ja"),
            IsoCode::ZH => write!(f, "zh"),
            IsoCode::AR => write!(f, "ar"),
            IsoCode::FA => write!(f, "fa"),
        }
    }
}

impl TryFrom<&str> for IsoCode {
    type Error = DspMetaError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "de" => Ok(IsoCode::DE),
            "en" => Ok(IsoCode::EN),
            "fr" => Ok(IsoCode::FR),
            "it" => Ok(IsoCode::IT),
            "es" => Ok(IsoCode::ES),
            "pt" => Ok(IsoCode::PT),
            "nl" => Ok(IsoCode::NL),
            "pl" => Ok(IsoCode::PL),
            "ru" => Ok(IsoCode::RU),
            "ja" => Ok(IsoCode::JA),
            "zh" => Ok(IsoCode::ZH),
            "ar" => Ok(IsoCode::AR),
            "fa" => Ok(IsoCode::FA),
            _ => Err(DspMetaError::CreateValueObject(
                "Creating an IsoCode failed because provided value is not allowed.".to_string(),
            )),
        }
    }
}
