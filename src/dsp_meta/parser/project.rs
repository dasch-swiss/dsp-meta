mod project_attributes;
mod project_blocks;

use std::collections::HashMap;

use hcl::Block;
use tracing::info;

use crate::domain::project::Project;
use crate::domain::value_objects::{
    AlternativeNames, CreatedAt, CreatedBy, Datasets, Description, EndDate, Funders, Grants,
    HowToCite, Name, ProjectValue, Shortcode, StartDate, TeaserText, ID,
};
use crate::errors::DspMetaError;
use crate::parser::project::project_blocks::parse_project_blocks;

pub fn parse_project(project_block: &Block) -> Result<Project, DspMetaError> {
    let project_label = project_block.labels().first().ok_or_else(|| {
        DspMetaError::ParseProject("Parse error: project needs to have one label.")
    })?;
    let id = ID::new(project_label.as_str());

    // extract the project attributes
    // created_at, created_by, shortcode, name, teaser_text, how_to_cite, start_date, end_date, datasets, funders, grants

    let attributes =
        project_attributes::parse_project_attributes(project_block.body.attributes().collect())?;

    let created_at = extract_created_at(&attributes)?;

    let created_by = extract_created_by(&attributes)?;

    let shortcode = extract_shortcode(&attributes)?;

    let name = extract_name(&attributes)?;

    let teaser_text = extract_teaser_text(&attributes)?;

    let how_to_cite = extract_how_to_cite(&attributes)?;

    let start_date = extract_start_date(&attributes)?;

    let end_date = extract_end_date(&attributes)?;

    let datasets = extract_datasets(&attributes)?;

    let funders = extract_funders(&attributes)?;

    let grants = extract_grants(&attributes)?;

    // extract the project blocks
    // alternative_names, description, url, keywords, disciplines, publications)

    let project_blocks: Vec<&Block> = project_block.body.blocks().collect();
    let _blocks = parse_project_blocks(project_blocks)?;

    let alternative_names = AlternativeNames::new(HashMap::new());
    let description = Description::new(HashMap::new());

    let project = Project::new(
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
        datasets,
        funders,
        Some(grants),
    );

    Ok(project)
}

fn extract_created_at(attributes: &HashMap<&str, ProjectValue>) -> Result<CreatedAt, DspMetaError> {
    let created_at_value = attributes.get("created_at").ok_or_else(|| {
        DspMetaError::ParseProject("Parse error: project needs to have a created_at value.")
    })?;

    let mut created_at: CreatedAt = Default::default();
    if let ProjectValue::CreatedAt(v) = created_at_value {
        created_at = v.clone();
    }
    Ok(created_at)
}

fn extract_created_by(attributes: &HashMap<&str, ProjectValue>) -> Result<CreatedBy, DspMetaError> {
    let created_by_value = attributes.get("created_by").ok_or_else(|| {
        DspMetaError::ParseProject("Parse error: project needs to have a created_by value.")
    })?;

    let mut created_by = Default::default();
    if let ProjectValue::CreatedBy(v) = created_by_value {
        created_by = v.clone();
    }
    Ok(created_by)
}

fn extract_shortcode(attributes: &HashMap<&str, ProjectValue>) -> Result<Shortcode, DspMetaError> {
    let shortcode_value = attributes.get("shortcode").ok_or_else(|| {
        DspMetaError::ParseProject("Parse error: project needs to have a shortcode.")
    })?;

    // FIXME: Feels a bit hacky, to get the value object out of the enum in this way.
    let mut shortcode = Default::default();
    if let ProjectValue::Shortcode(v) = shortcode_value {
        shortcode = v.clone();
    }
    Ok(shortcode)
}

fn extract_name(attributes: &HashMap<&str, ProjectValue>) -> Result<Name, DspMetaError> {
    let name_value = attributes
        .get("name")
        .ok_or_else(|| DspMetaError::ParseProject("Parse error: project needs to have a name."))?;

    let mut name = Default::default();
    if let ProjectValue::Name(v) = name_value {
        name = v.clone();
    }
    Ok(name)
}

fn extract_teaser_text(
    attributes: &HashMap<&str, ProjectValue>,
) -> Result<TeaserText, DspMetaError> {
    let teaser_text_value = attributes.get("teaser_text").ok_or_else(|| {
        DspMetaError::ParseProject("Parse error: project needs to have a teaser_text.")
    })?;

    let mut teaser_text = Default::default();
    if let ProjectValue::TeaserText(v) = teaser_text_value {
        teaser_text = v.clone();
    }
    Ok(teaser_text)
}

fn extract_how_to_cite(
    attributes: &HashMap<&str, ProjectValue>,
) -> Result<HowToCite, DspMetaError> {
    let how_to_cite_value = attributes.get("how_to_cite").ok_or_else(|| {
        DspMetaError::ParseProject("Parse error: project needs to have a how_to_cite.")
    })?;

    let mut how_to_cite = Default::default();
    if let ProjectValue::HowToCite(v) = how_to_cite_value {
        how_to_cite = v.clone();
    }
    Ok(how_to_cite)
}

fn extract_start_date(attributes: &HashMap<&str, ProjectValue>) -> Result<StartDate, DspMetaError> {
    let start_date_value = attributes.get("start_date").ok_or_else(|| {
        DspMetaError::ParseProject("Parse error: project needs to have a start_date.")
    })?;

    let mut start_date = Default::default();
    if let ProjectValue::StartDate(v) = start_date_value {
        start_date = v.clone();
    }
    Ok(start_date)
}

fn extract_end_date(
    attributes: &HashMap<&str, ProjectValue>,
) -> Result<Option<EndDate>, DspMetaError> {
    let maybe_end_date_value = attributes.get("end_date");

    let mut end_date: Option<EndDate> = Default::default();
    match maybe_end_date_value {
        None => {
            info!("Project does not have an 'end_date' defined.");
            end_date = None
        }
        Some(end_date_value) => {
            if let ProjectValue::EndDate(v) = end_date_value {
                end_date = Some(v.clone());
            }
        }
    }
    Ok(end_date)
}

fn extract_datasets(attributes: &HashMap<&str, ProjectValue>) -> Result<Datasets, DspMetaError> {
    let datasets_value = attributes.get("datasets").ok_or_else(|| {
        DspMetaError::ParseProject("Parse error: project needs to have datasets.")
    })?;

    let mut datasets = Default::default();
    if let ProjectValue::Datasets(v) = datasets_value {
        datasets = v.clone();
    }
    Ok(datasets)
}

fn extract_funders(attributes: &HashMap<&str, ProjectValue>) -> Result<Funders, DspMetaError> {
    let funders_value = attributes
        .get("funders")
        .ok_or_else(|| DspMetaError::ParseProject("Parse error: project needs to have funders."))?;

    let mut funders = Default::default();
    if let ProjectValue::Funders(v) = funders_value {
        funders = v.clone();
    }
    Ok(funders)
}

fn extract_grants(attributes: &HashMap<&str, ProjectValue>) -> Result<Grants, DspMetaError> {
    let grants_value = attributes
        .get("grants")
        .ok_or_else(|| DspMetaError::ParseProject("Parse error: project needs to have grants."))?;

    let mut grants = Default::default();
    if let ProjectValue::Grants(v) = grants_value {
        grants = v.clone();
    }
    Ok(grants)
}

#[cfg(test)]
mod tests {

    use tracing_test::traced_test;

    use super::*;
    use crate::parser::project::project_attributes::parse_project_attributes;

    #[traced_test]
    #[test]
    fn parse_project() {
        let input = r#"
            project "0803" {
                created_at = 1630601274523025000
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
                datasets = ["dataset-1"]
                funders = ["funder-1"]
                grants = []
            }
        "#;

        let body: hcl::Body = hcl::from_str(input).unwrap();
        let blocks: Vec<&hcl::Block> = body.blocks().collect();
        let project = super::parse_project(blocks.first().unwrap()).unwrap();
        dbg!(&project);
        assert_eq!(project.id(), &ID::new("0803"));
        assert_eq!(project.created_at(), &CreatedAt::new(1630601274523025000));
        assert_eq!(project.created_by(), &CreatedBy::new("dsp-metadata-gui"));
        assert_eq!(project.shortcode(), &Shortcode::new("0803"));
        assert_eq!(
            project.name(),
            &Name::new("The German Family Panel (pairfam)")
        );
        assert_eq!(
            project.teaser_text(),
            &TeaserText::new(
                "The German Family Panel (pairfam) is a multidisciplinary, longitudinal study."
            )
        );
        assert_eq!(
            project.how_to_cite(),
            &HowToCite::new(
                "Huinink, Johannes; Schröder, Carolin; Castiglioni, Laura; Feldhaus, Michael"
            )
        );
        assert_eq!(project.start_date(), &StartDate::new("2009-04-01"));
        assert_eq!(project.end_date(), &Some(EndDate::new("2012-03-31")));
    }

    #[traced_test]
    #[test]
    fn warn_on_unknown_attribute_or_block() {
        let input = r#"
            project "0803" {
                shortcode = "0803"
                gugus = "gugus"
            }
        "#;

        let body: hcl::Body = hcl::from_str(input).unwrap();
        let blocks: Vec<&hcl::Block> = body.blocks().collect();
        let project_block = blocks.first().unwrap();
        let attributes: Vec<&hcl::Attribute> = project_block.body().attributes().collect();
        let _ = parse_project_attributes(attributes);

        assert!(logs_contain("Parse error: unknown attribute 'gugus'"));
    }
}
