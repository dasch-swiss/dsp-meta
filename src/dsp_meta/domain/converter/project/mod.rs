use std::collections::HashMap;

use hcl::Block;

use crate::domain::converter::project::project_blocks::parse_project_blocks;
use crate::domain::project::Project;
use crate::domain::{AlternativeNames, Description, ID};
use crate::errors::DspMetaError;

mod project_attributes;
mod project_blocks;

pub fn convert_project_block(project_block: &Block) -> Result<Project, DspMetaError> {
    if project_block.identifier.as_str() != "project" {
        return Err(crate::errors::DspMetaError::ParseProject(
            "Parse error: project block needs to be named 'project'.",
        ));
    }

    let project_label = project_block.labels().first().ok_or_else(|| {
        DspMetaError::ParseProject("Parse error: project needs to have one label.")
    })?;
    let id = ID(project_label.as_str());

    // extract the project attributes
    // created_at, created_by, shortcode, name, teaser_text, how_to_cite, start_date, end_date, datasets, funders, grants

    let extracted_attributes =
        project_attributes::extract_project_attributes(project_block.body.attributes().collect())?;

    let created_at = extracted_attributes.created_at.ok_or_else(|| {
        DspMetaError::ParseProject("Parse error: project needs to have a created_at value.")
    })?;

    let created_by = extracted_attributes.created_by.ok_or_else(|| {
        DspMetaError::ParseProject("Parse error: project needs to have a created_by value.")
    })?;

    let shortcode = extracted_attributes.shortcode.ok_or_else(|| {
        DspMetaError::ParseProject("Parse error: project needs to have a shortcode.")
    })?;

    let name = extracted_attributes
        .name
        .ok_or_else(|| DspMetaError::ParseProject("Parse error: project needs to have a name."))?;

    let teaser_text = extracted_attributes.teaser_text.ok_or_else(|| {
        DspMetaError::ParseProject("Parse error: project needs to have a teaser_text.")
    })?;

    let how_to_cite = extracted_attributes.how_to_cite.ok_or_else(|| {
        DspMetaError::ParseProject("Parse error: project needs to have a how_to_cite.")
    })?;

    let start_date = extracted_attributes.start_date.ok_or_else(|| {
        DspMetaError::ParseProject("Parse error: project needs to have a start_date.")
    })?;

    let end_date = extracted_attributes.end_date;

    // extract the project blocks
    // alternative_names, description, url, keywords, disciplines, publications)

    let project_blocks: Vec<&Block> = project_block.body.blocks().collect();
    let _blocks = parse_project_blocks(project_blocks)?;

    let alternative_names = AlternativeNames(HashMap::new());
    let description = Description(HashMap::new());

    let project = Project {
        id,
        created_at,
        created_by,
        shortcode,
        name,
        alternative_names,
        teaser_text,
        description,
        how_to_cite,
        start_date,
        end_date,
    };

    Ok(project)
}

#[cfg(test)]
mod tests {
    use hcl::block;
    use tracing_test::traced_test;

    use crate::domain::{
        CreatedAt, CreatedBy, EndDate, HowToCite, Name, Shortcode, StartDate, TeaserText,
    };

    use super::*;

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
            }
        );
        let project = super::convert_project_block(&input_project_block).unwrap();
        dbg!(&project);
        assert_eq!(project.id, ID("0803"));
        assert_eq!(project.created_at, CreatedAt(1630601274523025000));
        assert_eq!(project.created_by, CreatedBy("dsp-metadata-gui"));
        assert_eq!(project.shortcode, Shortcode("0803"));
        assert_eq!(project.name, Name("The German Family Panel (pairfam)"));
        assert_eq!(
            project.teaser_text,
            TeaserText(
                "The German Family Panel (pairfam) is a multidisciplinary, longitudinal study."
            )
        );
        assert_eq!(
            project.how_to_cite,
            HowToCite(
                "Huinink, Johannes; Schröder, Carolin; Castiglioni, Laura; Feldhaus, Michael"
            )
        );
        assert_eq!(project.start_date, StartDate("2009-04-01"));
        assert_eq!(project.end_date, Some(EndDate("2012-03-31")));
    }
}
