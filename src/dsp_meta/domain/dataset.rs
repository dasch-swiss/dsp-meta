use crate::domain::Title;
use crate::errors::DspMetaError;

#[derive(Debug, PartialEq)]
pub struct Dataset {
    pub title: Title,
}

impl TryFrom<&hcl::Block> for Dataset {
    type Error = DspMetaError;

    fn try_from(dataset_block: &hcl::Block) -> Result<Self, Self::Error> {
        if dataset_block.identifier.as_str() != "dataset" {
            return Err(DspMetaError::ParseDataset(
                "Parse error: dataset block needs to be named 'dataset'.",
            ));
        }

        let title = Title(String::from("TODO: implement title extraction"));
        Ok(Self { title })
    }
}
