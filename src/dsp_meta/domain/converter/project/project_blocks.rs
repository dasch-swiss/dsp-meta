use crate::domain::{AlternativeName, AlternativeNames, Description};
use crate::errors::DspMetaError;

struct ExtractedProjectBlocks<'a> {
    pub alternative_names: AlternativeNames<'a>,
    pub description: Option<Description<'a>>,
}

pub fn extract_project_blocks(
    blocks: Vec<&hcl::Block>,
) -> Result<ExtractedProjectBlocks, DspMetaError> {
    for block in blocks {
        let mut alternative_names: Vec<&AlternativeName> = vec![];

        match block.identifier.as_str() {
            "alternative_name" => {
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
    Ok(ExtractedProjectBlocks {
        alternative_names: AlternativeNames::default(),
        description: None,
    })
}
