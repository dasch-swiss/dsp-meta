use dsp_domain::metadata::entity::dataset::Dataset;
use dsp_domain::metadata::entity::project::Project;
use dsp_domain::metadata::entity::project_metadata::ProjectMetadata;
use dsp_domain::metadata::value::version::Version;

use crate::api::convert::hcl::hcl_attribute::HclAttribute;
use crate::api::convert::hcl::hcl_block::HclBlock;
use crate::api::convert::hcl::hcl_body::HclBody;
use crate::error::DspMetaError;

impl<'a> TryInto<ProjectMetadata> for HclBody<'a> {
    type Error = DspMetaError;

    /// Converts an `hcl::Body` into `ProjectMetadata` by consuming the
    /// input. This operation can fail.
    fn try_into(self) -> Result<ProjectMetadata, Self::Error> {
        let mut maybe_version: Option<Version> = None;
        let mut maybe_project: Option<Project> = None;
        let mut maybe_datasets: Vec<Dataset> = vec![];

        let attributes: Vec<&hcl::Attribute> = self.0.attributes().collect();
        for attribute in attributes {
            match attribute.key() {
                "version" => maybe_version = Some(HclAttribute(attribute).try_into()?),
                _ => {
                    continue;
                }
            }
        }

        let blocks: Vec<&hcl::Block> = self.0.blocks().collect();
        for block in blocks {
            match block.identifier() {
                "project" => {
                    if maybe_project.is_some() {
                        return Err(DspMetaError::ParseProject(
                            "Only one project block allowed.".to_string(),
                        ));
                    } else {
                        maybe_project = Some(HclBlock(block).try_into()?)
                    }
                }
                "dataset" => maybe_datasets.push(HclBlock(block).try_into()?),
                _ => {
                    continue;
                }
            }
        }

        // Validate that the version is provided
        let version = maybe_version.ok_or_else(|| {
            DspMetaError::ParseVersion("Version attribute is not provided.".to_string())
        })?;

        // Validate that the project is provided
        let project = maybe_project.ok_or_else(|| {
            DspMetaError::ParseProject("Project block is not provided.".to_string())
        })?;

        // Validate that at least one dataset is provided
        let datasets = if !maybe_datasets.is_empty() {
            maybe_datasets
        } else {
            return Err(DspMetaError::ParseProject(
                "At least one dataset block needs to be provided.".to_string(),
            ));
        };

        // Validate that all referenced datasets exist
        for dataset in &project.datasets {
            if !datasets.iter().any(|d| d.id == *dataset) {
                return Err(DspMetaError::ParseProject(format!(
                    "Dataset with id '{:?}' referenced in project block does not exist.",
                    dataset
                )));
            }
        }

        let metadata = ProjectMetadata {
            version,
            project,
            datasets,
            grants: Vec::new(),
            organizations: Vec::new(),
            persons: Vec::new(),
        };
        Ok(metadata)
    }
}

#[cfg(test)]
mod tests {
    use dsp_domain::metadata::entity::project_metadata::ProjectMetadata;
    use hcl::body;

    use crate::api::convert::hcl::hcl_body::HclBody;
    use crate::error::DspMetaError;

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

        let project: Result<ProjectMetadata, DspMetaError> = HclBody(&input).try_into();
        assert!(project.is_err());
    }

    #[test]
    fn try_from_no_project_error() {
        let input = body!();

        let project: Result<ProjectMetadata, DspMetaError> = HclBody(&input).try_into();
        assert!(project.is_err());
    }
}
