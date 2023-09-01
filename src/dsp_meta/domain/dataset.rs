use crate::domain::{Title, ID};
use crate::errors::DspMetaError;

#[derive(Debug, PartialEq)]
pub struct Dataset<'a> {
    pub id: ID<'a>,
    pub title: Title<'a>,
}

impl<'a> TryFrom<&hcl::Block> for Dataset<'a> {
    type Error = DspMetaError;

    fn try_from(dataset_block: &hcl::Block) -> Result<Self, Self::Error> {
        if dataset_block.identifier.as_str() != "dataset" {
            return Err(DspMetaError::ParseDataset(
                "Parse error: dataset block needs to be named 'dataset'.",
            ));
        }

        let dataset_label = dataset_block.labels().first().ok_or_else(|| {
            DspMetaError::ParseDataset("Parse error: dataset needs to have one label.")
        })?;
        let id = ID(dataset_label.as_str());

        let title = Title("TODO: implement title extraction");
        Ok(Self { id, title })
    }
}
