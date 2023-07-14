use hcl::Block;

use crate::errors::DspMetaError;

pub fn parse_project_blocks(blocks: Vec<&Block>) -> Result<(), DspMetaError> {
    for block in blocks {
        dbg!(block);
    }
    Ok(())
}
