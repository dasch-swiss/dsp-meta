use std::collections::HashSet;
use std::fmt::{Debug, Display, Formatter};
use std::hash::Hash;

use hcl::Block;
use url::Url as UrlString;

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
                            "Only one project block allowed.",
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
            version: version
                .ok_or_else(|| DspMetaError::ParseVersion("Version attribute is not provided."))?,
            project: project
                .ok_or_else(|| DspMetaError::ParseProject("Project block is not provided."))?,
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
pub struct AlternativeName(HashSet<LangString>);

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
        let mut set = HashSet::new();
        for name in names {
            set.insert(name);
        }
        Self(set)
    }
}

/// Represents a string in a specific language.
/// Equality of two language specific strings is ONLY based iso_code.
#[derive(Debug, Clone)]
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

impl Hash for LangString {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.iso_code.hash(state);
    }
}

impl PartialEq for LangString {
    fn eq(&self, other: &Self) -> bool {
        self.iso_code == other.iso_code
    }
}

impl Eq for LangString {}

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
                "Creating an IsoCode failed because provided value is not allowed.",
            )),
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct TeaserText(String);

/// A set of descriptions in different languages.
#[derive(Debug, Clone, PartialEq)]
pub struct Description(HashSet<LangString>);

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
        let mut set = HashSet::new();
        for value in values {
            set.insert(value);
        }
        Self(set)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct UrlValue {
    pub value: UrlString,
    pub text: String,
}

impl Default for UrlValue {
    fn default() -> Self {
        UrlValue {
            value: UrlString::try_from("https://default.xyz").unwrap(),
            text: "Default URL description".to_string(),
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct HowToCite(String);

#[derive(Debug, Default, PartialEq)]
pub struct Keyword(HashSet<LangString>);
impl From<Vec<LangString>> for Keyword {
    fn from(values: Vec<LangString>) -> Self {
        let mut set = HashSet::new();
        for value in values {
            set.insert(value);
        }
        Self(set)
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct StartDate(String);

#[derive(Debug, Default, Clone, PartialEq)]
pub struct EndDate(String);

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
    url: UrlValue,
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
