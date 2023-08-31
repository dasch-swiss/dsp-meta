use std::collections::HashMap;

use hcl::Block;

use crate::errors::DspMetaError;

struct ExtractedProjectBlocks<'a> {
    pub alternative_names: Vec<AlterntiveName>,
    pub description: Option<Description<'a>>,
}

pub fn parse_project_blocks(
    blocks: Vec<&Block>,
) -> Result<HashMap<&str, ProjectValue>, DspMetaError> {
    let result: HashMap<&str, ProjectValue> = HashMap::new();

    for block in blocks {
        match block.identifier.as_str() {
            "alternative_name" => {
                // TODO: how do we handle multiple alternative names?
                println!("alternative_name");
                dbg!(block);
            }
            "description" => {
                println!("description");
                dbg!(block);
            }
            _ => {
                // catch all
                println!("catch all");
            }
        }
    }
    Ok(result)
}
