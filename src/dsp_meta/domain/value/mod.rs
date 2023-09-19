use std::collections::HashMap;

use iso_code::IsoCode;

use crate::errors::DspMetaError;

pub(crate) mod discipline;
pub(crate) mod iso_code;
mod lang_text_data;
mod ref_data;
pub(crate) mod spatial_coverage;
pub(crate) mod temporal_coverage;
pub(crate) mod version;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct ID(String);

#[derive(Debug, Default, Clone, PartialEq)]
pub struct CreatedAt(pub u64);

#[derive(Debug, Default, Clone, PartialEq)]
pub struct CreatedBy(pub String);

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Shortcode(pub String);

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Name(pub String);

/// A HashSet of an alternative name in different languages.
/// The HashSet is used to ensure that there are no duplicate values in regards
/// to the language code, as LangString only compares the iso_code.
/// TODO: check if this is the correct data structure
#[derive(Debug, Clone, PartialEq)]
pub struct AlternativeName(HashMap<IsoCode, String>);

impl Default for AlternativeName {
    fn default() -> Self {
        Self::from(vec![
            LangString {
                iso_code: IsoCode::DE,
                string: String::from("Der Default AlternativeName."),
            },
            LangString {
                iso_code: IsoCode::EN,
                string: String::from("The default AlternativeName."),
            },
            LangString {
                iso_code: IsoCode::FR,
                string: String::from("Le default AlternativeName."),
            },
        ])
    }
}

impl From<Vec<LangString>> for AlternativeName {
    fn from(names: Vec<LangString>) -> Self {
        let mut map = HashMap::new();
        for name in names {
            map.insert(name.iso_code, name.string);
        }
        Self(map)
    }
}

/// Represents a string in a specific language.
/// Equality of two language specific strings is ONLY based iso_code.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LangString {
    pub iso_code: IsoCode,
    pub string: String,
}

impl Default for LangString {
    fn default() -> Self {
        Self {
            iso_code: IsoCode::DE,
            string: String::from("Der Default LangString."),
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct TeaserText(pub String);

/// A set of descriptions in different languages.
#[derive(Debug, Clone, PartialEq)]
pub struct Description(HashMap<IsoCode, String>);

impl Default for Description {
    fn default() -> Self {
        Self::from(vec![
            LangString {
                iso_code: IsoCode::DE,
                string: String::from("Die Default-Beschreibung."),
            },
            LangString {
                iso_code: IsoCode::EN,
                string: String::from("The default description."),
            },
            LangString {
                iso_code: IsoCode::FR,
                string: String::from("Le standard description."),
            },
        ])
    }
}

impl From<Vec<LangString>> for Description {
    fn from(values: Vec<LangString>) -> Self {
        let mut map = HashMap::new();
        for value in values {
            map.insert(value.iso_code, value.string);
        }
        Self(map)
    }
}

/// Represents an HCL attribute which consists of an attribute key and a value expression.
///
/// In HCL syntax this is represented as:
///
/// ```hcl
/// url  "value" {
///    text = "text"
/// }
/// ```
///
/// Use [`Attribute::new`] to construct an [`Attribute`] from a value that is convertible to this
/// crate's [`Expression`] type.
#[derive(Debug, PartialEq, Eq)]
pub struct URL {
    pub value: url::Url,
    pub description: String,
}

impl URL {
    pub fn new(value: String, description: String) -> Result<Self, DspMetaError> {
        let maybe_url = url::Url::try_from(value.as_str());
        match maybe_url {
            Ok(value) => Ok(URL { value, description }),
            Err(_) => Err(DspMetaError::CreateValueObject(
                "Creating an UrlValue failed because provided value is not a valid URL."
                    .to_string(),
            )),
        }
    }
}

impl Default for URL {
    fn default() -> Self {
        URL {
            value: url::Url::try_from("https://default.xyz").unwrap(),
            description: "Default URL description".to_string(),
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct HowToCite(pub String);

#[derive(Debug, Default, PartialEq)]
pub struct Keyword(HashMap<IsoCode, String>);

impl From<Vec<LangString>> for Keyword {
    fn from(values: Vec<LangString>) -> Self {
        let mut map = HashMap::new();
        for value in values {
            map.insert(value.iso_code, value.string);
        }
        Self(map)
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct StartDate(pub String);

#[derive(Debug, Default, Clone, PartialEq)]
pub struct EndDate(pub String);

#[derive(Debug, Default, PartialEq)]
pub struct ContactPoint(pub String);

#[derive(Debug, Default, PartialEq)]
pub struct Title(pub String);

#[derive(Debug, PartialEq)]
pub struct Publication(String);

impl Default for Publication {
    fn default() -> Self {
        Publication("Default publication".to_string())
    }
}
