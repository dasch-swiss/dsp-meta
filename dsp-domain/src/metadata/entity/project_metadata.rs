use serde::Serialize;

use crate::metadata::entity::dataset::Dataset;
use crate::metadata::entity::grant::Grant;
use crate::metadata::entity::organization::Organization;
use crate::metadata::entity::person::Person;
use crate::metadata::entity::project::Project;
use crate::metadata::value::version::Version;

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
