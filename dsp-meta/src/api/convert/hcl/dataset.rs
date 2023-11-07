use dsp_domain::metadata::entity::dataset::Dataset;
use dsp_domain::metadata::value::Title;

use crate::api::convert::hcl::hcl_block::HclBlock;
use crate::error::DspMetaError;

impl<'a> TryInto<Dataset> for HclBlock<'a> {
    type Error = DspMetaError;

    fn try_into(self) -> Result<Dataset, Self::Error> {
        if self.0.identifier.as_str() != "dataset" {
            return Err(DspMetaError::ParseDataset(
                format!(
                    "Parse error: dataset block needs to be named 'dataset', however got '{}' instead.",
                    self.0.identifier.as_str()
                )
                    .to_string(),
            ));
        }

        let title = Title(String::from("TODO: implement title extraction"));
        Ok(Dataset { title })
    }
}
