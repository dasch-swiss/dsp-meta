mod project;
mod version;

use hcl::Block;

use crate::converter::project::parse_project;
use crate::converter::version::parse_version;
use crate::domain::Metadata;
use crate::errors::DspMetaError;

/// The function parses the body of HCL.
pub fn parse(body: hcl::Body) -> Result<Metadata, DspMetaError> {
    let version = parse_version(body.attributes().collect())?;

    let blocks: Vec<&Block> = body.blocks().collect();
    let project_block = extract_project_block(blocks)?;
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

fn extract_project_block(blocks: Vec<&Block>) -> Result<&Block, DspMetaError> {
    let mut project_block: Vec<&Block> = vec![];

    for block in blocks {
        if block.identifier.as_str() == "project" {
            project_block.push(block);
        }
    }

    if project_block.len() > 1 {
        return Err(DspMetaError::ParseProject(
            "There can only be one project block.",
        ));
    }

    if project_block.is_empty() {
        return Err(DspMetaError::ParseProject(
            "There needs to be a project block.",
        ));
    }

    Ok(project_block[0])
}

#[cfg(test)]
mod tests {
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
        let project = super::parse(body);
        assert!(project.is_err());
    }

    #[test]
    fn parse_no_project_error() {
        let input = r#"
        "#;

        let body: hcl::Body = hcl::from_str(input).unwrap();
        let project = super::parse(body);
        assert!(project.is_err());
    }
}
