use dsp_domain::metadata::value::keyword::Keyword;
use dsp_domain::metadata::value::lang_text_data::LangTextData;

use crate::api::convert::hcl::hcl_attribute::HclAttributes;
use crate::api::convert::hcl::hcl_block::HclBlock;
use crate::error::DspMetaError;

const KEYWORD_BLOCK_IDENTIFIER: &str = "keyword";

impl<'a> TryInto<Keyword> for HclBlock<'a> {
    type Error = DspMetaError;

    fn try_into(self) -> Result<Keyword, Self::Error> {
        if self.0.identifier.as_str() != KEYWORD_BLOCK_IDENTIFIER {
            let msg = format!(
                "The passed block is not named correctly. Expected '{}', however got '{}' instead.",
                KEYWORD_BLOCK_IDENTIFIER,
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
    use dsp_domain::metadata::value::keyword::*;

    use crate::api::convert::hcl::hcl_block::HclBlock;
    use crate::error::DspMetaError;

    #[test]
    fn test_try_from_correct_block() {
        let block = hcl::block!(
            keyword {
                de = "Der keyword"
                en = "The keyword"
                fr = "Le keyword"
            }
        );

        let keyword: Keyword = HclBlock(&block).try_into().unwrap();

        let mut map: HashMap<IsoCode, String> = HashMap::new();
        map.insert(IsoCode::DE, String::from("Der keyword"));
        map.insert(IsoCode::EN, String::from("The keyword"));
        map.insert(IsoCode::FR, String::from("Le keyword"));
        let expected = Keyword(map);

        assert_eq!(keyword, expected);
    }

    #[test]
    fn test_try_from_incorrect_block() {
        let block = hcl::block!(
            keyword_other {
                de = "Der keyword"
                en = "The keyword"
                fr = "Le keyword"
            }
        );

        let keyword: Result<Keyword, DspMetaError> = HclBlock(&block).try_into();

        assert!(keyword.is_err());
    }
}
