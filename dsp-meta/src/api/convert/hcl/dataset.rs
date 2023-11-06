use crate::domain::model::entity::dataset::Dataset;
use crate::domain::model::value::Title;
use crate::error::DspMetaError;

impl TryFrom<&hcl::Block> for Dataset {
    type Error = DspMetaError;

    fn try_from(dataset_block: &hcl::Block) -> Result<Self, Self::Error> {
        if dataset_block.identifier.as_str() != "dataset" {
            return Err(DspMetaError::ParseDataset(
                format!(
                    "Parse error: dataset block needs to be named 'dataset', however got '{}' instead.",
                    dataset_block.identifier.as_str()
                )
                .to_string(),
            ));
        }

        let title = Title(String::from("TODO: implement title extraction"));
        Ok(Self { title })
    }
}
