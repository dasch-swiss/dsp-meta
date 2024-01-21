use dsp_domain::metadata::entity::dataset::Dataset;
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

        let abstracts = if !extracted_blocks.abstracts.is_empty() {
            Ok(extracted_blocks.abstracts)
        } else {
            Err(DspMetaError::ParseDataset(
                "Parse dataset: there needs to be at least one abstract.".to_string(),
            ))
        }?;

        let licenses = if !extracted_blocks.licenses.is_empty() {
            Ok(extracted_blocks.licenses)
        } else {
            Err(DspMetaError::ParseDataset(
                "Parse dataset: there needs to be at least one license.".to_string(),
            ))
        }?;

        let languages = if !extracted_blocks.languages.is_empty() {
            Ok(extracted_blocks.languages)
        } else {
            Err(DspMetaError::ParseDataset(
                "Parse dataset: there needs to be at least one language.".to_string(),
            ))
        }?;

        let attributions = if !extracted_blocks.attributions.is_empty() {
            Ok(extracted_blocks.attributions)
        } else {
            Err(DspMetaError::ParseDataset(
                "Parse dataset: there needs to be at least one attribution.".to_string(),
            ))
        }?;
        let urls = extracted_blocks.urls;

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
            abstracts,
            licenses,
            languages,
            attributions,
            urls,
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
                abstract {
                    en = "The interdisciplinary research project \"The image sequences of Basel's early prints: Late Medieval didactic didactics as an image-text reading\" combines a comprehensive art scholarly analysis of the links between images and texts in the illustrated incunabula in Basel with the digitization of the holdings of the University Library and the development of an electronic edition in the form of a new kind of Web-0.2 application. The project is carried out by Kunsthistorische Seminar of the University of Basel (Prof. Dr. B. Schellewald) and Digital Humanities Lab of the University of Basel (Prof. Dr. L. Rosenthaler).  The core of the digital edition consists of around twenty richly illustrated early prints from four different Basel officers. Many of them appeared in several editions before 1500, some of them in German and Latin at almost the same time. It is an extraordinarily varied production; in addition to the Mirror of Salvation, there is a novel, the Melusine, the travelogues of Jean de Mandeville, some prayer and edification books, theological writings, Lent sermons, the lives of Saints Fridolin and Meinrad, the famous ship of fools and the knight of Thurn.  The Internet publication makes the digitized corpus of these early prints usable for the scientific edition as well as for the exploration of images and texts through the possibilities of non-linear linking and commenting on the images and texts. Existing and emerging online editions can also be linked to it, which optimises the use of databases from other institutions with regard to our corpus."
                }
                license {
                    type  = "creative_commons"
                    href  = "https://creativecommons.org/licenses/by-nc/4.0"
                    date  = "2021-09-02"
                    label = "CC BY-NC 4.0"
                }
                language {
                    de = "Deutsch"
                    en = "German"
                    fr = "Allemand"
                }

                // reference to person or organization
                attribution {
                    agent = "http://ns.dasch.swiss/repository#dsp-081C-organization-000" // reference to person or organization
                    roles = ["Author"]
                }
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
