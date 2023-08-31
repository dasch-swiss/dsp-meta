use std::collections::HashMap;
use std::fmt::Debug;

use hcl::Block;
use serde::{Deserialize, Serialize};

use crate::domain::dataset::Dataset;
use crate::domain::grant::Grant;
use crate::domain::organization::Organization;
use crate::domain::person::Person;
use crate::domain::project::Project;
use crate::domain::version::Version;
use crate::errors::DspMetaError;

mod converter;
mod dataset;
mod grant;
mod organization;
mod person;
mod project;
mod version;

/// The Metadata struct represents the metadata of a DSP project.
/// TODO: check if the cardinality of the fields are correct
#[derive(Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct Metadata {
    pub version: Version,
    pub project: Project,
    pub datasets: Vec<Dataset>,
    pub grants: Vec<Grant>,
    pub organizations: Vec<Organization>,
    pub persons: Vec<Person>,
}

impl TryFrom<&hcl::Body> for Metadata {
    type Error = crate::errors::DspMetaError;

    fn try_from(body: &hcl::Body) -> Result<Self, Self::Error> {
        let mut version: Option<Version> = None;
        let mut projects: Vec<Project> = vec![];
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
                "project" => projects.push(Project::try_from(block)?),
                "dataset" => datasets.push(Dataset::try_from(block)?),
                _ => {
                    continue;
                }
            }
        }

        let metadata = Metadata {
            version: version
                .ok_or_else(|| DspMetaError::ParseVersion("Version attribute is not provided."))?,
            project: Project::try_from(projects)?,
            datasets: Vec::new(),
            grants: Vec::new(),
            organizations: Vec::new(),
            persons: Vec::new(),
        };
        Ok(metadata)
    }
}

#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
pub struct ID<'a>(&'a str);

#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
pub struct CreatedAt(pub u64);

#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
pub struct CreatedBy<'a>(&'a str);

#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
pub struct Shortcode<'a>(&'a str);

#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
pub struct Name<'a>(&'a str);

/// A HashMap of language codes and their corresponding alternative names.
#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
pub struct AlternativeNames(HashMap<String, String>);

pub struct AlternativeName {}

/// Represents a string in a specific language.
pub struct LangString<'a> {
    pub iso_code: IsoCode,
    pub string: &'a str,
}

/// Language codes according to ISO 639-1
/// Not an exhaustive list.
enum IsoCode {
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

#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
pub struct TeaserText<'a>(&'a str);

/// A HashMap of language codes and their corresponding descriptions.
#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
pub struct Description(HashMap<String, String>);

#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
pub struct HowToCite<'a>(&'a str);

#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
pub struct StartDate<'a>(&'a str);

#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
pub struct EndDate<'a>(&'a str);

#[derive(Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct Datasets(Vec<Dataset>);

#[derive(Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct Grants(Vec<Grant>);

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
