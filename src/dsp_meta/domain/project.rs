use serde::{Deserialize, Serialize};

use crate::domain::{
    AlternativeNames, CreatedAt, CreatedBy, Description, EndDate, HowToCite, Name, Shortcode,
    StartDate, TeaserText, ID,
};

// no need for smart constructors here, as there is no validation happening
#[derive(Debug, Default, PartialEq, Deserialize, Serialize)]
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

impl TryFrom<hcl::Block> for Project {
    type Error = crate::errors::DspMetaError;

    fn try_from(project_block: hcl::Block) -> Result<Self, Self::Error> {
        if project_block.identifier.as_str() != "project" {
            Err(crate::errors::DspMetaError::ParseProject(
                "Parse error: project block needs to be named 'project'.",
            ))
        } else {
            let project = Project {
                shortcode: Shortcode::new("test"),
                ..Project::default()
            };
            Ok(project)
        }
    }
}

#[cfg(test)]
mod tests {
    use hcl::block;

    use super::*;
    use crate::domain::Shortcode;

    #[test]
    fn test_try_from_block() {
        let project_block = block!(
            project "test" {
                shortcode = "test"
            }
        );
        let project = Project::try_from(project_block).unwrap();
        assert_eq!(project.shortcode, Shortcode::new("test"));
    }
}
