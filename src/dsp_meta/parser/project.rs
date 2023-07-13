use std::collections::HashMap;

use hcl::{Block, Expression};
use tracing::warn;

use crate::domain::value_objects::{
    CreatedAt, CreatedBy, Datasets, EndDate, Funders, Grants, HowToCite, ProjectValues, Shortcode,
    StartDate, TeaserText, ID,
};
use crate::domain::Project;
use crate::errors::DspMetaError;

pub fn parse_project(project_block: &Block) -> Result<Project, DspMetaError> {
    let project_label = project_block.labels().first().ok_or_else(|| {
        DspMetaError::ParseProject("Parse error: project needs to have one label.")
    })?;
    let id = ID::new(project_label.as_str());

    let attributes = parse_project_attributes(project_block.body.attributes().collect())?;

    let created_at = extract_created_at(&attributes)?;

    let created_by = extract_created_by(&attributes)?;

    let shortcode = extract_shortcode(&attributes)?;

    let teaser_text = extract_teaser_text(&attributes)?;

    let how_to_cite = extract_how_to_cite(&attributes)?;

    let start_date = extract_start_date(&attributes)?;

    let end_date = extract_end_date(&attributes)?;

    let datasets = extract_datasets(&attributes)?;

    let funders = extract_funders(&attributes)?;

    let grants = extract_grants(&attributes)?;

    let project_blocks: Vec<&Block> = project_block.body.blocks().collect();
    for block in project_blocks {
        dbg!(block);
    }

    let project = Project::new(
        id,
        created_at,
        created_by,
        shortcode,
        teaser_text,
        how_to_cite,
        start_date,
        end_date,
        datasets,
        funders,
        grants,
    );

    Ok(project)
}

fn parse_project_attributes(
    attributes: Vec<&hcl::Attribute>,
) -> Result<HashMap<&str, ProjectValues>, DspMetaError> {
    let mut results: HashMap<&str, ProjectValues> = HashMap::new();

    for attribute in attributes {
        match attribute.key() {
            "created_at" => {
                let created_at = match attribute.expr() {
                    Expression::Number(value) => Ok(ProjectValues::CreatedAt(CreatedAt::new(
                        value.as_u64().unwrap(),
                    ))),
                    _ => Err(DspMetaError::ParseProject(
                        "Parse error: created_at needs to be a number.",
                    )),
                }?;
                results.insert("created_at", created_at);
            }
            "created_by" => {
                let created_by = match attribute.expr() {
                    Expression::String(value) => {
                        Ok(ProjectValues::CreatedBy(CreatedBy::new(value)))
                    }
                    _ => Err(DspMetaError::ParseProject(
                        "Parse error: created_by needs to be a string.",
                    )),
                }?;
                results.insert("created_by", created_by);
            }
            "shortcode" => {
                let shortcode = match attribute.expr() {
                    Expression::String(value) => {
                        Ok(ProjectValues::Shortcode(Shortcode::new(value)))
                    }
                    _ => Err(DspMetaError::ParseProject(
                        "Parse error: shortcode needs to be a string.",
                    )),
                }?;
                results.insert("shortcode", shortcode);
            }
            "teaser_text" => {
                let teaser_text = match attribute.expr() {
                    Expression::String(value) => {
                        Ok(ProjectValues::TeaserText(TeaserText::new(value)))
                    }
                    _ => Err(DspMetaError::ParseProject(
                        "Parse error: teaser_text needs to be a string.",
                    )),
                }?;
                results.insert("teaser_text", teaser_text);
            }
            "how_to_cite" => {
                let how_to_cite = match attribute.expr() {
                    Expression::String(value) => {
                        Ok(ProjectValues::HowToCite(HowToCite::new(value)))
                    }
                    _ => Err(DspMetaError::ParseProject(
                        "Parse error: how_to_cite needs to be a string.",
                    )),
                }?;
                results.insert("how_to_cite", how_to_cite);
            }
            "start_date" => {
                let start_date = match attribute.expr() {
                    Expression::String(value) => {
                        Ok(ProjectValues::StartDate(StartDate::new(value)))
                    }
                    _ => Err(DspMetaError::ParseProject(
                        "Parse error: start_date needs to be a string.",
                    )),
                }?;
                results.insert("start_date", start_date);
            }
            "end_date" => {
                let end_date = match attribute.expr() {
                    Expression::String(value) => Ok(ProjectValues::EndDate(EndDate::new(value))),
                    _ => Err(DspMetaError::ParseProject(
                        "Parse error: end_date needs to be a string.",
                    )),
                }?;
                results.insert("end_date", end_date);
            }
            "datasets" => {
                let datasets = match attribute.expr() {
                    Expression::Array(value) => {
                        let mut datasets_value: Vec<String> = Vec::new();
                        for element in value {
                            match element {
                                Expression::String(value) => {
                                    datasets_value.push(value.to_string());
                                }
                                _ => {
                                    return Err(DspMetaError::ParseProject(
                                        "Parse error: datasets needs to be a list of strings.",
                                    ))
                                }
                            }
                        }
                        Ok(ProjectValues::Datasets(Datasets::new(datasets_value)))
                    }
                    _ => Err(DspMetaError::ParseProject(
                        "Parse error: datasets needs to be a list of strings.",
                    )),
                }?;
                results.insert("datasets", datasets);
            }
            "funders" => {
                let funders = match attribute.expr() {
                    Expression::Array(value) => {
                        let mut funders_value: Vec<String> = Vec::new();
                        for element in value {
                            match element {
                                Expression::String(value) => {
                                    funders_value.push(value.to_string());
                                }
                                _ => {
                                    return Err(DspMetaError::ParseProject(
                                        "Parse error: funders needs to be a list of strings.",
                                    ))
                                }
                            }
                        }
                        Ok(ProjectValues::Funders(Funders::new(funders_value)))
                    }
                    _ => Err(DspMetaError::ParseProject(
                        "Parse error: funders needs to be a list of strings.",
                    )),
                }?;
                results.insert("funders", funders);
            }
            "grants" => {
                let grants = match attribute.expr() {
                    Expression::Array(value) => {
                        let mut grants_value: Vec<String> = Vec::new();
                        for element in value {
                            match element {
                                Expression::String(value) => {
                                    grants_value.push(value.to_string());
                                }
                                _ => {
                                    return Err(DspMetaError::ParseProject(
                                        "Parse error: grants needs to be a list of strings.",
                                    ))
                                }
                            }
                        }
                        Ok(ProjectValues::Grants(Grants::new(grants_value)))
                    }
                    _ => Err(DspMetaError::ParseProject(
                        "Parse error: grants needs to be a list of strings.",
                    )),
                }?;
                results.insert("grants", grants);
            }
            _ => {
                warn!("Parse error: unknown attribute '{}'.", attribute.key());
            }
        }
    }
    Ok(results)
}

fn extract_created_at(
    attributes: &HashMap<&str, ProjectValues>,
) -> Result<CreatedAt, DspMetaError> {
    let created_at_value = attributes.get("created_at").ok_or_else(|| {
        DspMetaError::ParseProject("Parse error: project needs to have a created_at value.")
    })?;

    let mut created_at: CreatedAt = Default::default();
    if let ProjectValues::CreatedAt(v) = created_at_value {
        created_at = v.clone();
    }
    Ok(created_at)
}

fn extract_created_by(
    attributes: &HashMap<&str, ProjectValues>,
) -> Result<CreatedBy, DspMetaError> {
    let created_by_value = attributes.get("created_by").ok_or_else(|| {
        DspMetaError::ParseProject("Parse error: project needs to have a created_by value.")
    })?;

    let mut created_by = Default::default();
    if let ProjectValues::CreatedBy(v) = created_by_value {
        created_by = v.clone();
    }
    Ok(created_by)
}

fn extract_shortcode(attributes: &HashMap<&str, ProjectValues>) -> Result<Shortcode, DspMetaError> {
    let shortcode_value = attributes.get("shortcode").ok_or_else(|| {
        DspMetaError::ParseProject("Parse error: project needs to have a shortcode.")
    })?;

    // FIXME: This is feels a bit hacky to get the value object out of the enum.
    let mut shortcode = Default::default();
    if let ProjectValues::Shortcode(v) = shortcode_value {
        shortcode = v.clone();
    }
    Ok(shortcode)
}

fn extract_teaser_text(
    attributes: &HashMap<&str, ProjectValues>,
) -> Result<TeaserText, DspMetaError> {
    let teaser_text_value = attributes.get("teaser_text").ok_or_else(|| {
        DspMetaError::ParseProject("Parse error: project needs to have a teaser_text.")
    })?;

    let mut teaser_text = Default::default();
    if let ProjectValues::TeaserText(v) = teaser_text_value {
        teaser_text = v.clone();
    }
    Ok(teaser_text)
}

fn extract_how_to_cite(
    attributes: &HashMap<&str, ProjectValues>,
) -> Result<HowToCite, DspMetaError> {
    let how_to_cite_value = attributes.get("how_to_cite").ok_or_else(|| {
        DspMetaError::ParseProject("Parse error: project needs to have a how_to_cite.")
    })?;

    let mut how_to_cite = Default::default();
    if let ProjectValues::HowToCite(v) = how_to_cite_value {
        how_to_cite = v.clone();
    }
    Ok(how_to_cite)
}

fn extract_start_date(
    attributes: &HashMap<&str, ProjectValues>,
) -> Result<StartDate, DspMetaError> {
    let start_date_value = attributes.get("start_date").ok_or_else(|| {
        DspMetaError::ParseProject("Parse error: project needs to have a start_date.")
    })?;

    let mut start_date = Default::default();
    if let ProjectValues::StartDate(v) = start_date_value {
        start_date = v.clone();
    }
    Ok(start_date)
}

fn extract_end_date(attributes: &HashMap<&str, ProjectValues>) -> Result<EndDate, DspMetaError> {
    let end_date_value = attributes.get("end_date").ok_or_else(|| {
        DspMetaError::ParseProject("Parse error: project needs to have a end_date.")
    })?;

    let mut end_date = Default::default();
    if let ProjectValues::EndDate(v) = end_date_value {
        end_date = v.clone();
    }
    Ok(end_date)
}

fn extract_datasets(attributes: &HashMap<&str, ProjectValues>) -> Result<Datasets, DspMetaError> {
    let datasets_value = attributes.get("datasets").ok_or_else(|| {
        DspMetaError::ParseProject("Parse error: project needs to have datasets.")
    })?;

    let mut datasets = Default::default();
    if let ProjectValues::Datasets(v) = datasets_value {
        datasets = v.clone();
    }
    Ok(datasets)
}

fn extract_funders(attributes: &HashMap<&str, ProjectValues>) -> Result<Funders, DspMetaError> {
    let funders_value = attributes
        .get("funders")
        .ok_or_else(|| DspMetaError::ParseProject("Parse error: project needs to have funders."))?;

    let mut funders = Default::default();
    if let ProjectValues::Funders(v) = funders_value {
        funders = v.clone();
    }
    Ok(funders)
}

fn extract_grants(attributes: &HashMap<&str, ProjectValues>) -> Result<Grants, DspMetaError> {
    let grants_value = attributes
        .get("grants")
        .ok_or_else(|| DspMetaError::ParseProject("Parse error: project needs to have grants."))?;

    let mut grants = Default::default();
    if let ProjectValues::Grants(v) = grants_value {
        grants = v.clone();
    }
    Ok(grants)
}

#[cfg(test)]
mod tests {

    use tracing_test::traced_test;

    use crate::domain::value_objects::{
        CreatedAt, CreatedBy, EndDate, HowToCite, Shortcode, StartDate, TeaserText, ID,
    };

    #[traced_test]
    #[test]
    fn parse_project() {
        let input = r#"
            project "0803" {
                created_at = 1630601274523025000
                created_by  = "dsp-metadata-gui"
                shortcode = "0803"
                teaser_text = "The German Family Panel (pairfam) is a multidisciplinary, longitudinal study."
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
        assert_eq!(project.end_date(), &EndDate::new("2012-03-31"));
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
        let _ = super::parse_project_attributes(attributes);

        assert!(logs_contain("Parse error: unknown attribute 'gugus'"));
    }
}
