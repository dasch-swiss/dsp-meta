use dsp_domain::metadata::value::publication::Publication;
use dsp_domain::metadata::value::simple_text_data::SimpleTextData;

use crate::api::convert::hcl::hcl_attribute::HclAttributes;
use crate::api::convert::hcl::hcl_block::HclBlock;
use crate::error::DspMetaError;

const PUBLICATION_BLOCK_IDENTIFIER: &str = "publication";

impl<'a> TryInto<Publication> for HclBlock<'a> {
    type Error = DspMetaError;

    fn try_into(self) -> Result<Publication, Self::Error> {
        if self.0.identifier.as_str() != PUBLICATION_BLOCK_IDENTIFIER {
            let msg = format!(
                "The passed block is not named correctly. Expected 'publication', however got '{}' instead.",
                self.0.identifier.as_str()
            );
            return Err(DspMetaError::CreateValueObject(msg));
        }

        let attributes: Vec<&hcl::Attribute> = self.0.body.attributes().collect();
        let simple_text_data: Result<SimpleTextData, DspMetaError> =
            HclAttributes(attributes).try_into();
        simple_text_data.map(|s| s.into_simple_text())
    }
}

#[cfg(test)]
mod tests {
    use dsp_domain::metadata::value::publication::*;
    use dsp_domain::metadata::value::simple_text_data::SimpleTextData;

    use crate::api::convert::hcl::hcl_block::HclBlock;
    use crate::error::DspMetaError;

    #[test]
    fn test_try_from_correct_block() {
        let block = hcl::block!(
            publication {
                text = "A publication"
            }
        );

        let publication: Publication = HclBlock(&block).try_into().unwrap();

        match publication {
            Publication::SimpleText(data) => {
                assert_eq!(data, SimpleTextData("A publication".to_string()));
            }
        }
    }

    #[test]
    fn test_try_from_incorrect_block() {
        let block = hcl::block!(
            publication_other {
                text = "A publication"
            }
        );

        let publication: Result<Publication, DspMetaError> = HclBlock(&block).try_into();

        assert!(publication.is_err());
    }
}
