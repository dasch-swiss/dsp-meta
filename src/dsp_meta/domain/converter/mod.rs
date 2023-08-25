use hcl::Block;

use crate::errors::DspMetaError;

pub mod project;

pub fn extract_project_block(blocks: Vec<&Block>) -> Result<&Block, DspMetaError> {
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
