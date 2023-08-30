use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Dataset {
    pub id: String,
    pub title: String,
}

impl TryFrom<hcl::Block> for Dataset {
    type Error = crate::errors::DspMetaError;

    fn try_from(dataset_block: hcl::Block) -> Result<Self, Self::Error> {
        if dataset_block.identifier.as_str() != "dataset" {
            return Err(crate::errors::DspMetaError::ParseDataset(
                "Parse error: dataset block needs to be named 'dataset'.",
            ));
        }
        let id = dataset_block.labels().unwrap().to_string();
        let title = dataset_block
            .attributes()
            .next()
            .unwrap()
            .value()
            .to_string();
        Ok(Self { id, title })
    }
}
