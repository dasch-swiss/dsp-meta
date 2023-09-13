use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};
use std::hash::Hash;

use hcl::Block;

use crate::domain::dataset::Dataset;
use crate::domain::grant::Grant;
use crate::domain::organization::Organization;
use crate::domain::person::Person;
use crate::domain::project::Project;
use crate::domain::version::Version;
use crate::errors::DspMetaError;

mod convert;
mod dataset;
mod grant;
mod organization;
mod person;
mod project;
mod version;

/// The Metadata struct represents the metadata of a DSP project.
#[derive(Debug, Default, PartialEq)]
pub struct Metadata {
    pub version: Version,
    pub project: Project,
    pub datasets: Vec<Dataset>,
    pub grants: Vec<Grant>,
    pub organizations: Vec<Organization>,
    pub persons: Vec<Person>,
}

impl TryFrom<&hcl::Body> for Metadata {
    type Error = DspMetaError;

    fn try_from(body: &hcl::Body) -> Result<Self, Self::Error> {
        let mut version: Option<Version> = None;
        let mut project: Option<Project> = None;
        let mut datasets: Vec<Dataset> = vec![];

        let attributes: Vec<&hcl::Attribute> = body.attributes().collect();
        for attribute in attributes {
            match attribute.key() {
                "version" => version = Some(Version::try_from(attribute)?),
                _ => {
                    continue;
                }
            }
        }

        let blocks: Vec<&Block> = body.blocks().collect();
        for block in blocks {
            match block.identifier() {
                "project" => {
                    if project.is_some() {
                        return Err(DspMetaError::ParseProject(
                            "Only one project block allowed.".to_string(),
                        ));
                    } else {
                        project = Some(Project::try_from(block)?)
                    }
                }
                "dataset" => datasets.push(Dataset::try_from(block)?),
                _ => {
                    continue;
                }
            }
        }

        let metadata = Metadata {
            version: version.ok_or_else(|| {
                DspMetaError::ParseVersion("Version attribute is not provided.".to_string())
            })?,
            project: project.ok_or_else(|| {
                DspMetaError::ParseProject("Project block is not provided.".to_string())
            })?,
            datasets: Vec::new(),
            grants: Vec::new(),
            organizations: Vec::new(),
            persons: Vec::new(),
        };
        Ok(metadata)
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct ID(String);

#[derive(Debug, Default, Clone, PartialEq)]
pub struct CreatedAt(pub u64);

#[derive(Debug, Default, Clone, PartialEq)]
pub struct CreatedBy(String);

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Shortcode(String);

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Name(String);

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

/// Language codes according to ISO 639-1
/// Not an exhaustive list.
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
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

#[derive(Debug, Default, Clone, PartialEq)]
pub struct TeaserText(String);

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
pub struct HowToCite(String);

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
pub struct StartDate(String);

#[derive(Debug, Default, Clone, PartialEq)]
pub struct EndDate(String);

#[derive(Debug, Default, PartialEq)]
pub struct ContactPoint(String);

#[derive(Debug, Default, PartialEq)]
pub struct Datasets(Vec<Dataset>);

#[derive(Debug, Default, PartialEq)]
pub struct Grants(Vec<Grant>);

#[derive(Debug, Default, PartialEq)]
pub struct Title(String);

#[derive(Debug, Default, PartialEq)]
pub struct Discipline {
    discipline_type: DisciplineType,
    description: LangString,
    url: URL,
}

#[derive(Debug, Default, PartialEq)]
pub enum DisciplineType {
    #[default]
    Snf,
}

#[derive(Debug, PartialEq)]
pub struct Publication(String);

impl Default for Publication {
    fn default() -> Self {
        Publication("Default publication".to_string())
    }
}

#[cfg(test)]
mod tests {
    use hcl::body;

    use super::*;

    #[test]
    fn try_from_multiple_projects_error() {
        let input = body!(
            project {
                shortcode = "0803"
            }
            project {
                shortcode = "0804"
            }
        );

        let project = Metadata::try_from(&input);
        assert!(project.is_err());
    }

    #[test]
    fn try_from_no_project_error() {
        let input = body!();

        let project = Metadata::try_from(&input);
        assert!(project.is_err());
    }
}
