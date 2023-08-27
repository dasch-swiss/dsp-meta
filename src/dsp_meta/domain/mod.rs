mod converter;
mod dataset;
mod grant;
mod organization;
mod person;
mod project;
mod version;

use std::collections::HashMap;
use std::fmt::Debug;

use hcl::Block;
use serde::{Deserialize, Serialize};

use crate::domain::converter::extract_project_block;
use crate::domain::converter::project::convert_project;
use crate::domain::dataset::Dataset;
use crate::domain::grant::Grant;
use crate::domain::organization::Organization;
use crate::domain::person::Person;
use crate::domain::project::Project;
use crate::domain::version::Version;

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

impl TryFrom<hcl::Body> for Metadata {
    type Error = crate::errors::DspMetaError;

    fn try_from(body: hcl::Body) -> Result<Self, Self::Error> {
        let attributes: Vec<&hcl::Attribute> = body.attributes().collect();
        let version = Version::try_from(attributes)?;

        let blocks: Vec<&Block> = body.blocks().collect();
        let project_block = extract_project_block(blocks)?;
        let project = convert_project(project_block)?;
        dbg!(&project);

        let metadata = Metadata {
            version,
            project,
            datasets: Vec::new(),
            grants: Vec::new(),
            organizations: Vec::new(),
            persons: Vec::new(),
        };
        Ok(metadata)
    }
}

/// Denotes possible values for a project.
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub enum ProjectValue {
    ID(ID),
    CreatedAt(CreatedAt),
    CreatedBy(CreatedBy),
    Shortcode(Shortcode),
    Name(Name),
    AlternativeNames(AlternativeNames),
    TeaserText(TeaserText),
    Description(Description),
    HowToCite(HowToCite),
    StartDate(StartDate),
    EndDate(EndDate),
    Datasets(Datasets),
    Funders(Funders),
    Grants(Grants),
}

#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
pub struct ID {
    pub id: String,
}

impl From<&str> for ID {
    fn from(id: &str) -> Self {
        Self { id: id.to_string() }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
pub struct CreatedAt(u64);
impl CreatedAt {
    pub fn new(created_at: u64) -> Self {
        Self(created_at)
    }
    pub fn value(&self) -> u64 {
        self.0
    }
}

#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
pub struct CreatedBy(String);
impl CreatedBy {
    pub fn new(created_by: &str) -> Self {
        Self(created_by.to_string())
    }
    pub fn value(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
pub struct Shortcode(String);
impl Shortcode {
    pub fn new(shortcode: &str) -> Self {
        Self(shortcode.to_string())
    }
    pub fn value(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
pub struct Name(String);
impl Name {
    pub fn new(name: &str) -> Self {
        Self(name.to_string())
    }
    pub fn value(&self) -> &str {
        &self.0
    }
}

/// A HashMap of language codes and their corresponding alternative names.
#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
pub struct AlternativeNames(HashMap<String, String>);
impl AlternativeNames {
    pub fn new(alternative_names: HashMap<String, String>) -> Self {
        Self(alternative_names)
    }
    pub fn value(&self) -> HashMap<String, String> {
        self.0.clone()
    }
}

#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
pub struct TeaserText(String);
impl TeaserText {
    pub fn new(teaser_text: &str) -> Self {
        Self(teaser_text.to_string())
    }
    pub fn value(&self) -> &str {
        &self.0
    }
}

/// A HashMap of language codes and their corresponding descriptions.
#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
pub struct Description(HashMap<String, String>);
impl Description {
    pub fn new(description: HashMap<String, String>) -> Self {
        Self(description)
    }
    pub fn value(&self) -> HashMap<String, String> {
        self.0.clone()
    }
}

#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
pub struct HowToCite(String);
impl HowToCite {
    pub fn new(how_to_cite: &str) -> Self {
        Self(how_to_cite.to_string())
    }
    pub fn value(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
pub struct StartDate(String);
impl StartDate {
    pub fn new(start_date: &str) -> Self {
        Self(start_date.to_string())
    }
    pub fn value(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
pub struct EndDate(String);
impl EndDate {
    pub fn new(end_date: &str) -> Self {
        Self(end_date.to_string())
    }
    pub fn value(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
pub struct Datasets(Vec<String>);
impl Datasets {
    pub fn new(datasets: Vec<String>) -> Self {
        Self(datasets)
    }
    pub fn value(&self) -> Vec<String> {
        self.0.clone()
    }
}

#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
pub struct Funders(Vec<String>);
impl Funders {
    pub fn new(funders: Vec<String>) -> Self {
        Self(funders)
    }
    pub fn value(&self) -> Vec<String> {
        self.0.clone()
    }
}

#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
pub struct Grants(Vec<String>);
impl Grants {
    pub fn new(grants: Vec<String>) -> Self {
        Self(grants)
    }
    pub fn value(&self) -> Vec<String> {
        self.0.clone()
    }
}

#[cfg(test)]
mod tests {
    use hcl::body;

    use crate::domain::Metadata;

    #[test]
    fn try_from_multiple_projects_error() {
        let input = body!(
            project "0803" {
                shortcode = "0803"
            }
            project "0804" {
                shortcode = "0804"
            }
        );

        let project = Metadata::try_from(input);
        assert!(project.is_err());
    }

    #[test]
    fn try_from_no_project_error() {
        let input = body!();

        let project = Metadata::try_from(input);
        assert!(project.is_err());
    }
}
