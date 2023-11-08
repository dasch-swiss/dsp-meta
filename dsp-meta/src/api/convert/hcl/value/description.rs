use dsp_domain::metadata::value::description::Description;
use dsp_domain::metadata::value::lang_text_data::LangTextData;

use crate::api::convert::hcl::hcl_attribute::HclAttributes;
use crate::api::convert::hcl::hcl_block::HclBlock;
use crate::error::DspMetaError;

const DESCRIPTION_BLOCK_IDENTIFIER: &str = "description";

impl<'a> TryInto<Description> for HclBlock<'a> {
    type Error = DspMetaError;

    fn try_into(self) -> Result<Description, Self::Error> {
        if self.0.identifier.as_str() != DESCRIPTION_BLOCK_IDENTIFIER {
            let msg = format!(
                "The passed block is not named correctly. Expected '{}', however got '{}' instead.",
                DESCRIPTION_BLOCK_IDENTIFIER,
                self.0.identifier.as_str()
            );
            return Err(DspMetaError::CreateValueObject(msg));
        }

        let attributes: Vec<&hcl::Attribute> = self.0.body.attributes().collect();

        // FIXME: improve API
        let lang_text_data: Result<LangTextData, DspMetaError> =
            HclAttributes(attributes).try_into();
        lang_text_data.map(|l| l.into())
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use dsp_domain::metadata::value::description::*;
    use dsp_domain::metadata::value::iso_code::IsoCode;

    use crate::api::convert::hcl::hcl_block::HclBlock;
    use crate::error::DspMetaError;

    #[test]
    fn test_try_from_correct_block() {
        let block = hcl::block!(
            description {
                de = "Die Beschreibung"
                en = "The description"
                fr = "La description"
            }
        );

        let description: Description = HclBlock(&block).try_into().unwrap();

        let mut map = HashMap::new();
        map.insert(IsoCode::DE, String::from("Die Beschreibung"));
        map.insert(IsoCode::EN, String::from("The description"));
        map.insert(IsoCode::FR, String::from("La description"));
        let expected = Description(map);

        assert_eq!(description, expected);
    }

    #[test]
    fn test_try_from_incorrect_block() {
        let block = hcl::block!(
            description_other {
                de = "Die Beschreibung"
                en = "The description"
                fr = "La description"
            }
        );

        let description: Result<Description, DspMetaError> = HclBlock(&block).try_into();

        assert!(description.is_err());
    }
}
