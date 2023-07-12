use hcl::{Block, Expression};

use crate::domain::Project;
use crate::errors::DspMetaError;

pub fn parse_project(project: Vec<&Block>) -> Result<Project, DspMetaError> {
    if project.len() > 1 {
        return Err(DspMetaError::ParseProject("More than 1 project defined."));
    }
    if project.is_empty() {
        return Err(DspMetaError::ParseProject("No project defined."));
    }

    let project_block = *(project.first().unwrap());
    dbg!(project_block);

    let project_label = project_block.labels().first().ok_or_else(|| {
        DspMetaError::ParseProject("Parse error: project needs to have one label.")
    })?;
    let id = project_label.as_str();

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
