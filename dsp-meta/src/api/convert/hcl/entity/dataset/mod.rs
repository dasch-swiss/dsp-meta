use dsp_domain::metadata::entity::dataset::Dataset;
use dsp_domain::metadata::value::attribution::Attribution;
use dsp_domain::metadata::value::language::Language;
use dsp_domain::metadata::value::license::License;
use dsp_domain::metadata::value::r#abstract::Abstract;
use dsp_domain::metadata::value::url::Url;
use extracted_dataset_attributes::ExtractedDatasetAttributes;
use extracted_dataset_blocks::ExtractedDatasetBlocks;

use crate::api::convert::hcl::hcl_block::HclBlock;
use crate::error::DspMetaError;

pub(crate) mod extracted_dataset_attributes;
pub(crate) mod extracted_dataset_blocks;

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

        // extract the dataset attributes
        // id (1), created_at (1), created_by (1), title (1), status (1), access_conditions (1),
        // how_to_cite (1),  date_published (0-1), type_of_data (1-n),  alternative_titles (0-n),

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

        let status = extracted_attributes.status.take().ok_or_else(|| {
            DspMetaError::ParseDataset("Parse error: dataset needs to have a status".to_string())
        })?;

        let access_conditions = extracted_attributes
            .access_conditions
            .take()
            .ok_or_else(|| {
                DspMetaError::ParseDataset(
                    "Parse error: dataset needs to have a access_condition".to_string(),
                )
            })?;

        let how_to_cite = extracted_attributes.how_to_cite.take().ok_or_else(|| {
            DspMetaError::ParseDataset("Parse error: dataset needs to have how_to_cite".to_string())
        })?;

        let date_published = extracted_attributes.date_published.take();

        let type_of_data = if !extracted_attributes.type_of_data.is_empty() {
            Ok(extracted_attributes.type_of_data)
        } else {
            Err(DspMetaError::ParseDataset(
                "Parse dataset: there needs to be at least one type_of_data.".to_string(),
            ))
        }?;

        let alternative_titles = extracted_attributes.alternative_titles;

        // extract the dataset blocks
        // abstracts (1-n), licenses (1-n), languages (1-n), attributions (1-n),
        // urls (0-n),

        let blocks: Vec<&hcl::Block> = self.0.body.blocks().collect();
        let extracted_blocks = ExtractedDatasetBlocks::try_from(blocks)?;

        Ok(Dataset {
            id,
            created_at,
            created_by,
            title,
            status,
            access_conditions,
            how_to_cite,
            date_published,
            type_of_data,
            alternative_titles,
            abstracts: vec![Abstract::default()],
            licenses: vec![License::default()],
            languages: vec![Language::default()],
            attributions: vec![Attribution::default()],
            urls: vec![Url::default()],
        })
    }
}

#[cfg(test)]
mod tests {
    use dsp_domain::metadata::value::access::Access;
    use dsp_domain::metadata::value::data_type::DataType;
    use dsp_domain::metadata::value::identifier::DatasetId;
    use dsp_domain::metadata::value::status::Status;
    use dsp_domain::metadata::value::{CreatedAt, CreatedBy, DatePublished, HowToCite, Title};
    use hcl::block;
    use tracing_test::traced_test;

    use super::*;

    #[traced_test]
    #[test]
    fn test_convert_dataset_block() {
        let input_dataset_block = block!(
            dataset {
                id                = "d1"
                created_at        = 1630601274523025000u64 // FIXME: is there a more readable way to write an i64?
                created_by        = "dsp-metadata-gui"
                title             = "The German Family Panel (pairfam)"
                how_to_cite       = "pairfam"
                status            = "Ongoing"
                access_conditions = "Open"
                date_published    = 1630601274523025000u64
                type_of_data      = ["Image", "Text"]
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
        assert_eq!(dataset.how_to_cite, HowToCite("pairfam".to_string()));
        assert_eq!(dataset.status, Status::Ongoing);
        assert_eq!(dataset.access_conditions, Access::Open);
        assert_eq!(
            dataset.date_published,
            Some(DatePublished(1630601274523025000u64))
        );
        assert_eq!(dataset.type_of_data, vec![DataType::Image, DataType::Text]);
    }
}
