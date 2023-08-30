use serde::{Deserialize, Serialize};

use crate::domain::converter::project::convert_project_block;
use crate::domain::{
    AlternativeNames, CreatedAt, CreatedBy, Description, EndDate, HowToCite, Name, Shortcode,
    StartDate, TeaserText, ID,
};
use crate::errors::DspMetaError;

// no need for smart constructors here, as there is no validation happening
#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
pub struct Project {
    pub id: ID,
    pub created_at: CreatedAt,
    pub created_by: CreatedBy,
    pub shortcode: Shortcode,
    pub name: Name,
    pub alternative_names: AlternativeNames,
    pub teaser_text: TeaserText,
    pub description: Description,
    pub how_to_cite: HowToCite,
    pub start_date: StartDate,
    pub end_date: Option<EndDate>,
}

impl TryFrom<&hcl::Block> for Project {
    type Error = crate::errors::DspMetaError;

    fn try_from(project_block: &hcl::Block) -> Result<Self, Self::Error> {
        convert_project_block(project_block)
    }
}

/// Given potentially a list of projects, flatten to one project.
/// Our constraint is that there must be exactly one project defined per metadata file.
impl TryFrom<Vec<Project>> for Project {
    type Error = crate::errors::DspMetaError;

    fn try_from(projects: Vec<Project>) -> Result<Self, Self::Error> {
        if projects.len() > 1 {
            return Err(DspMetaError::ParseProject(
                "There can only be one project block.",
            ));
        }

        if projects.is_empty() {
            return Err(DspMetaError::ParseProject(
                "There needs to be a project block.",
            ));
        }

        Ok(projects[0].clone())
    }
}
