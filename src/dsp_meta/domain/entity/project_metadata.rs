use serde::Serialize;

use crate::domain::entity::dataset::Dataset;
use crate::domain::entity::grant::Grant;
use crate::domain::entity::organization::Organization;
use crate::domain::entity::person::Person;
use crate::domain::entity::project::Project;
use crate::domain::value::version::Version;
use crate::errors::DspMetaError;

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

impl TryFrom<&hcl::Body> for ProjectMetadata {
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

        let blocks: Vec<&hcl::Block> = body.blocks().collect();
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

        let metadata = ProjectMetadata {
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
