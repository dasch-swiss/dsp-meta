use dsp_domain::metadata::value::lang_text_data::LangTextData;
use dsp_domain::metadata::value::language::Language;

use crate::api::convert::hcl::hcl_attribute::HclAttributes;
use crate::api::convert::hcl::hcl_block::HclBlock;
use crate::error::DspMetaError;

const BLOCK_IDENTIFIER: &str = "language";

impl<'a> TryInto<Language> for HclBlock<'a> {
    type Error = DspMetaError;

    fn try_into(self) -> Result<Language, Self::Error> {
        if self.0.identifier.as_str() != BLOCK_IDENTIFIER {
            let msg = format!(
                "The passed block is not named correctly. Expected '{}', however got '{}' instead.",
                BLOCK_IDENTIFIER,
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
    use dsp_domain::metadata::value::language::Language;

    use crate::api::convert::hcl::hcl_block::HclBlock;
    use crate::error::DspMetaError;

    #[test]
    fn test_try_from_correct_block() {
        let block = hcl::block!(
            language {
                de = "Deutsch"
                en = "German"
                fr = "Allemand"
            }
        );

        let result: Language = HclBlock(&block).try_into().unwrap();

        let mut map = HashMap::new();
        map.insert(IsoCode::DE, String::from("Deutsch"));
        map.insert(IsoCode::EN, String::from("German"));
        map.insert(IsoCode::FR, String::from("Allemand"));
        let expected = Language(map);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_try_from_incorrect_block() {
        let block = hcl::block!(
            other_block {
                de = "Deutsch"
                en = "German"
                fr = "Allemand"
            }
        );

        let result: Result<Description, DspMetaError> = HclBlock(&block).try_into();

        assert!(result.is_err());
    }
}
