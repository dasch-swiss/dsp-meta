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
        let mut version: Option<Version> = None;
        let mut project: Option<Project> = None;
        let mut datasets: Vec<Dataset> = vec![];

        let attributes: Vec<&hcl::Attribute> = self.0.body.attributes().collect();
        for attribute in attributes {
            match attribute.key() {
                "version" => version = Some(HclAttribute(attribute).try_into()?),
                _ => {
                    continue;
                }
            }
        }

        let blocks: Vec<&hcl::Block> = self.0.body.blocks().collect();
        for block in blocks {
            match block.identifier() {
                "project" => {
                    if project.is_some() {
                        return Err(DspMetaError::ParseProject(
                            "Only one project block allowed.".to_string(),
                        ));
                    } else {
                        project = Some(HclBlock(block).try_into()?)
                    }
                }
                "dataset" => datasets.push(HclBlock(block).try_into()?),
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
