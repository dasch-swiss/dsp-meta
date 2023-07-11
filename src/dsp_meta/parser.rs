use hcl::{Attribute, Block, Expression};

use crate::domain::{Metadata, Project};
use crate::errors::DspMetaError;

/// The function parses the body of HCL.
pub fn parse(body: hcl::Body) -> Result<Metadata, DspMetaError> {
    dbg!(&body);

    let attributes: Vec<&Attribute> = body.attributes().collect();
    let version = parse_version(attributes)?;
    dbg!(version);

    let blocks: Vec<&Block> = body.blocks().collect();
    let mut project_block: Vec<&Block> = vec![];

    for block in blocks {
        if block.identifier.as_str() == "project" {
            project_block.push(block);
        }
    }

    let project = parse_project(project_block)?;
    dbg!(&project);

    let metadata = Metadata::new(
        version,
        project,
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
    );
    Ok(metadata)
}

fn parse_version(attributes: Vec<&Attribute>) -> Result<u64, DspMetaError> {
    let mut version: u64 = 0;
    for attribute in attributes {
        if attribute.key() == "version" {
            version = match attribute.expr() {
                Expression::Number(value) => Ok(value.as_u64().unwrap()),
                _ => Err(DspMetaError::ParseVersion("Version needs to be a number.")),
            }?
        }
    }
    Ok(version)
}

fn parse_project(project: Vec<&Block>) -> Result<Project, DspMetaError> {
    if project.len() > 1 {
        return Err(DspMetaError::ParseProject("More than 1 project defined."));
    }
    if project.is_empty() {
        return Err(DspMetaError::ParseProject("No project defined."));
    }

    let project_block = *(project.first().unwrap());
    dbg!(project_block);

    let mut id = "";
    let project_label = project_block.labels().first().ok_or_else(|| {
        DspMetaError::ParseProject("Parse error: project needs to have one label.")
    })?;
    id = project_label.as_str();

    let mut shortcode: &str = "";
    let project_attributes: Vec<&hcl::Attribute> = project_block.body.attributes().collect();
    for attribute in project_attributes {
        dbg!(attribute);
        match attribute.key() {
            "shortcode" => {
                shortcode = match attribute.expr() {
                    Expression::String(value) => Ok(value.as_str()),
                    _ => Err(DspMetaError::ParseProject(
                        "Parse error: shortcode needs to be a string.",
                    )),
                }?;
            }
            _ => {
                dbg!("Parse error: unknown attribute.");
            }
        }
    }

    let project_blocks: Vec<&hcl::Block> = project_block.body.blocks().collect();
    for block in project_blocks {
        dbg!(block);
    }

    let project = Project::new(id, shortcode, Vec::new(), Vec::new(), Vec::new());

    Ok(project)
}

#[cfg(test)]
mod tests {
    #[test]
    fn parse_version() {
        let input = r#"
            version = 1
        "#;

        let body: hcl::Body = hcl::from_str(input).unwrap();
        let attributes: Vec<&hcl::Attribute> = body.attributes().collect();
        let version = super::parse_version(attributes).unwrap();
        assert_eq!(version, 1);
    }

    #[test]
    fn parse_project() {
        let input = r#"
            project "0803" {
                shortcode = "0803"
            }
        "#;

        let body: hcl::Body = hcl::from_str(input).unwrap();
        let blocks: Vec<&hcl::Block> = body.blocks().collect();
        let project = super::parse_project(blocks).unwrap();
        assert_eq!(project.id(), "0803");
        assert_eq!(project.shortcode(), "0803");
    }

    #[test]
    fn parse_multiple_projects_error() {
        let input = r#"
            project "0803" {
                shortcode = "0803"
            }
            project "0804" {
                shortcode = "0804"
            }
        "#;

        let body: hcl::Body = hcl::from_str(input).unwrap();
        let blocks: Vec<&hcl::Block> = body.blocks().collect();
        let project = super::parse_project(blocks);
        assert!(project.is_err());
    }

    #[test]
    fn parse_no_project_error() {
        let input = r#"
        "#;

        let body: hcl::Body = hcl::from_str(input).unwrap();
        let blocks: Vec<&hcl::Block> = body.blocks().collect();
        let project = super::parse_project(blocks);
        assert!(project.is_err());
    }
}
