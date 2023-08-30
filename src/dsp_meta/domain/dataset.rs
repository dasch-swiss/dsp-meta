use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Dataset {
    pub id: String,
    pub title: String,
}

impl TryFrom<hcl::Block> for Dataset {
    type Error = crate::errors::DspMetaError;

    fn try_from(dataset_block: hcl::Block) -> Result<Self, Self::Error> {
        let id = dataset_block.label().unwrap().to_string();
        let title = dataset_block
            .attributes()
            .next()
            .unwrap()
            .value()
            .to_string();
        Ok(Self { id, title })
    }
}
