use dsp_domain::metadata::value::alternative_name::AlternativeName;
use dsp_domain::metadata::value::lang_text_data::LangTextData;

use crate::api::convert::hcl::hcl_attribute::HclAttributes;
use crate::api::convert::hcl::hcl_block::HclBlock;
use crate::error::DspMetaError;

const ALTERNATIVE_NAME_BLOCK_IDENTIFIER: &str = "alternative_name";

impl<'a> TryInto<AlternativeName> for HclBlock<'a> {
    type Error = DspMetaError;

    fn try_into(self) -> Result<AlternativeName, Self::Error> {
        if self.0.identifier.as_str() != ALTERNATIVE_NAME_BLOCK_IDENTIFIER {
            let msg = format!(
                "The passed block is not named correctly. Expected '{}', however got '{}' instead.",
                ALTERNATIVE_NAME_BLOCK_IDENTIFIER,
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

    use dsp_domain::metadata::value::iso_code::IsoCode;

    use super::*;

    #[test]
    fn test_try_into_from_correct_block() {
        let block = hcl::block!(
            alternative_name {
                de = "Der alternative Name"
                en = "The alternative name"
                fr = "Le alternative name"
            }
        );

        let alternative_name: AlternativeName = HclBlock(&block).try_into().unwrap();
        let mut map: HashMap<IsoCode, String> = HashMap::new();
        map.insert(IsoCode::DE, String::from("Der alternative Name"));
        map.insert(IsoCode::EN, String::from("The alternative name"));
        map.insert(IsoCode::FR, String::from("Le alternative name"));
        let expected = AlternativeName(map);

        assert_eq!(alternative_name, expected);
    }

    #[test]
    fn test_try_into_from_incorrect_block() {
        let block = hcl::block!(
            alternative_name_other {
                de = "Der alternative Name"
                en = "The alternative name"
                fr = "Le alternative name"
            }
        );

        let alternative_name: Result<AlternativeName, DspMetaError> = HclBlock(&block).try_into();

        assert!(alternative_name.is_err());
    }
}
