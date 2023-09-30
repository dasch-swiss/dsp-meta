use std::collections::HashMap;

use serde::Serialize;

use crate::domain::model::value::iso_code::IsoCode;
use crate::domain::model::value::lang_text_data::LangTextData;
use crate::errors::DspMetaError;

const KEYWORD_BLOCK_IDENTIFIER: &str = "keyword";

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
pub struct Keyword(HashMap<IsoCode, String>);

impl TryFrom<&hcl::Block> for Keyword {
    type Error = DspMetaError;

    fn try_from(block: &hcl::Block) -> Result<Self, Self::Error> {
        if block.identifier.as_str() != KEYWORD_BLOCK_IDENTIFIER {
            let msg = format!(
                "The passed block is not named correctly. Expected '{}', however got '{}' instead.",
                KEYWORD_BLOCK_IDENTIFIER,
                block.identifier.as_str()
            );
            return Err(DspMetaError::CreateValueObject(msg));
        }

        let attributes: Vec<&hcl::Attribute> = block.body.attributes().collect();
        LangTextData::try_from(attributes).map(|lang_text_data| Keyword(lang_text_data.0))
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_try_from_correct_block() {
        let block = hcl::block!(
            keyword {
                de = "Der keyword"
                en = "The keyword"
                fr = "Le keyword"
            }
        );

        let keyword = Keyword::try_from(&block).unwrap();

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

        let keyword = Keyword::try_from(&block);

        assert!(keyword.is_err());
    }
}
