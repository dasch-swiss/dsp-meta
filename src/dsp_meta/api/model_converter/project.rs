use crate::api::model_converter::extracted_project_attributes::ExtractedProjectAttributes;
use crate::api::model_converter::extracted_project_blocks::ExtractedProjectBlocks;
use crate::domain::model::entity::project::Project;
use crate::domain::model::value::url::Url;
use crate::errors::DspMetaError;

impl TryFrom<&hcl::Block> for Project {
    type Error = DspMetaError;

    fn try_from(project_block: &hcl::Block) -> Result<Self, Self::Error> {
        if project_block.identifier.as_str() != "project" {
            return Err(DspMetaError::ParseProject(
                format!(
                    "Parse error: project block needs to be named 'project', however got '{}' instead.",
                    project_block.identifier.as_str()
                )
                .to_string(),
            ));
        }

        // extract the project attributes
        // created_at, created_by, shortcode, name, teaser_text, how_to_cite, start_date, end_date,
        // datasets, funders, grants

        let attributes: Vec<&hcl::Attribute> = project_block.body.attributes().collect();

        let extracted_attributes = ExtractedProjectAttributes::try_from(attributes)?;

        let created_at = extracted_attributes.created_at.ok_or_else(|| {
            DspMetaError::ParseProject(
                "Parse error: project needs to have a created_at value.".to_string(),
            )
        })?;

        let created_by = extracted_attributes.created_by.ok_or_else(|| {
            DspMetaError::ParseProject(
                "Parse error: project needs to have a created_by value.".to_string(),
            )
        })?;

        let shortcode = extracted_attributes.shortcode.ok_or_else(|| {
            DspMetaError::ParseProject(
                "Parse error: project needs to have a shortcode.".to_string(),
            )
        })?;

        let name = extracted_attributes.name.ok_or_else(|| {
            DspMetaError::ParseProject("Parse error: project needs to have a name.".to_string())
        })?;

        let teaser_text = extracted_attributes.teaser_text.ok_or_else(|| {
            DspMetaError::ParseProject(
                "Parse error: project needs to have a teaser_text.".to_string(),
            )
        })?;

        let how_to_cite = extracted_attributes.how_to_cite.ok_or_else(|| {
            DspMetaError::ParseProject(
                "Parse error: project needs to have a how_to_cite.".to_string(),
            )
        })?;

        let start_date = extracted_attributes.start_date.ok_or_else(|| {
            DspMetaError::ParseProject(
                "Parse error: project needs to have a start_date.".to_string(),
            )
        })?;

        let end_date = extracted_attributes.end_date;
        let contact_point = extracted_attributes.contact_point;

        // extract the project blocks
        // alternative_names, description, url, keywords, disciplines, publications)

        let blocks: Vec<&hcl::Block> = project_block.body.blocks().collect();
        let extracted_blocks = ExtractedProjectBlocks::try_from(blocks)?;

        let alternative_names = extracted_blocks.alternative_names;
        let description = extracted_blocks.description.ok_or_else(|| {
            DspMetaError::ParseProject(
                "Parse error: project needs to have a description.".to_string(),
            )
        })?;
        let url = Url::default();
        let keywords = vec![];
        let disciplines = vec![];
        let publications = vec![];

        let project = Project {
            created_at,
            created_by,
            shortcode,
            name,
            alternative_names,
            teaser_text,
            description,
            url,
            how_to_cite,
            start_date,
            end_date,
            contact_point,
            keywords,
            disciplines,
            publications,
        };

        Ok(project)
    }
}

#[cfg(test)]
mod tests {
    use hcl::block;
    use tracing_test::traced_test;

    use super::*;
    use crate::domain::model::value::{
        ContactPoint, CreatedAt, CreatedBy, EndDate, HowToCite, Name, Shortcode, StartDate,
        TeaserText,
    };

    #[traced_test]
    #[test]
    fn test_convert_project_block() {
        let input_project_block = block!(
            project {
                created_at = 1630601274523025000u64 // FIXME: is there a more readable way to write an i64?
                created_by  = "dsp-metadata-gui"
                shortcode = "0803"
                name = "The German Family Panel (pairfam)"
                alternative_name "1" {
                    de = "Der deutsche Familienpanel (pairfam)"
                    en = "The German Family Panel (pairfam)"
                }
                teaser_text = "The German Family Panel (pairfam) is a multidisciplinary, longitudinal study."
                description {
                    de = "Der deutsche Familienpanel (pairfam) ist eine multidisziplinäre, längsschnittliche Studie."
                    en = "The German Family Panel (pairfam) is a multidisciplinary, longitudinal study."
                }
                how_to_cite = "Huinink, Johannes; Schröder, Carolin; Castiglioni, Laura; Feldhaus, Michael"
                start_date  = "2009-04-01"
                end_date    = "2012-03-31"
                contact_point = "project_organization"
            }
        );
        let project = Project::try_from(&input_project_block).unwrap();
        assert_eq!(project.created_at, CreatedAt(1630601274523025000));
        assert_eq!(
            project.created_by,
            CreatedBy(String::from("dsp-metadata-gui"))
        );
        assert_eq!(project.shortcode, Shortcode(String::from("0803")));
        assert_eq!(
            project.name,
            Name(String::from("The German Family Panel (pairfam)"))
        );
        assert_eq!(
            project.teaser_text,
            TeaserText(String::from(
                "The German Family Panel (pairfam) is a multidisciplinary, longitudinal study."
            ))
        );
        assert_eq!(
            project.how_to_cite,
            HowToCite(String::from(
                "Huinink, Johannes; Schröder, Carolin; Castiglioni, Laura; Feldhaus, Michael"
            ))
        );
        assert_eq!(project.start_date, StartDate(String::from("2009-04-01")));
        assert_eq!(project.end_date, Some(EndDate(String::from("2012-03-31"))));
        assert_eq!(
            project.contact_point,
            Some(ContactPoint(String::from("project_organization")))
        );
    }
}
