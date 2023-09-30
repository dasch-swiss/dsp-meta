use serde::Serialize;

use crate::domain::model::entity::dataset::Dataset;
use crate::domain::model::entity::grant::Grant;
use crate::domain::model::entity::organization::Organization;
use crate::domain::model::entity::person::Person;
use crate::domain::model::entity::project::Project;
use crate::domain::model::value::version::Version;

/// The Metadata struct represents the metadata of a DSP project.
#[derive(Debug, Default, Clone, PartialEq, Serialize)]
pub struct ProjectMetadata {
    pub version: Version,
    pub project: Project,
    pub datasets: Vec<Dataset>,
    pub grants: Vec<Grant>,
    pub organizations: Vec<Organization>,
    pub persons: Vec<Person>,
}
