mod project;
mod version;

use hcl::{Attribute, Block, Expression};

use crate::domain::{Metadata, Project};
use crate::errors::DspMetaError;
use crate::parser::project::parse_project;
use crate::parser::version::parse_version;

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

#[cfg(test)]
mod tests {}
