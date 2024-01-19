use dsp_domain::metadata::entity::dataset::Dataset;

use crate::api::convert::hcl::extracted_dataset_attributes::ExtractedDatasetAttributes;
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

        let attributes: Vec<&hcl::Attribute> = self.0.body.attributes().collect();

        let mut extracted_attributes = ExtractedDatasetAttributes::try_from(attributes)?;

        let id = extracted_attributes.id.take().ok_or_else(|| {
            DspMetaError::ParseDataset("Parse error: project needs to have an id.".to_string())
        })?;

        let created_at = extracted_attributes.created_at.take().ok_or_else(|| {
            DspMetaError::ParseDataset(
                "Parse error: project needs to have a created_at value.".to_string(),
            )
        })?;

        let created_by = extracted_attributes.created_by.take().ok_or_else(|| {
            DspMetaError::ParseDataset(
                "Parse error: project needs to have a created_by value.".to_string(),
            )
        })?;

        let title = extracted_attributes.title.take().ok_or_else(|| {
            DspMetaError::ParseDataset("Parse error: dataset needs to have a title.".to_string())
        })?;

        Ok(Dataset {
            id,
            created_at,
            created_by,
            title,
        })
    }
}

#[cfg(test)]
mod tests {
    use dsp_domain::metadata::value::identifier::DatasetId;
    use dsp_domain::metadata::value::{CreatedAt, CreatedBy, Title};
    use hcl::block;
    use tracing_test::traced_test;

    use super::*;

    #[traced_test]
    #[test]
    fn test_convert_dataset_block() {
        let input_dataset_block = block!(
            dataset {
                id = "d1"
                created_at = 1630601274523025000u64 // FIXME: is there a more readable way to write an i64?
                created_by  = "dsp-metadata-gui"
                title = "The German Family Panel (pairfam)"
            }
        );
        let dataset: Dataset = HclBlock(&input_dataset_block).try_into().unwrap();
        assert_eq!(dataset.id, DatasetId(String::from("d1")));
        assert_eq!(dataset.created_at, CreatedAt(1630601274523025000));
        assert_eq!(
            dataset.created_by,
            CreatedBy(String::from("dsp-metadata-gui"))
        );
        assert_eq!(
            dataset.title,
            Title(String::from("The German Family Panel (pairfam)"))
        );
    }
}
