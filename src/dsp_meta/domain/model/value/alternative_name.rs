use std::collections::HashMap;

use serde::Serialize;

use crate::domain::model::value::iso_code::IsoCode;
use crate::domain::model::value::lang_text_data::LangTextData;
use crate::errors::DspMetaError;

const ALTERNATIVE_NAME_BLOCK_IDENTIFIER: &str = "alternative_name";

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct AlternativeName(HashMap<IsoCode, String>);

impl Default for AlternativeName {
    fn default() -> Self {
        let mut map: HashMap<IsoCode, String> = HashMap::new();
        map.insert(IsoCode::DE, String::from("Der Default AlternativeName."));
        map.insert(IsoCode::EN, String::from("The default AlternativeName."));
        map.insert(IsoCode::FR, String::from("Le default AlternativeName."));
        Self(map)
    }
}

impl TryFrom<&hcl::Block> for AlternativeName {
    type Error = DspMetaError;

    fn try_from(block: &hcl::Block) -> Result<Self, Self::Error> {
        if block.identifier.as_str() != ALTERNATIVE_NAME_BLOCK_IDENTIFIER {
            let msg = format!(
                "The passed block is not named correctly. Expected '{}', however got '{}' instead.",
                ALTERNATIVE_NAME_BLOCK_IDENTIFIER,
                block.identifier.as_str()
            );
            return Err(DspMetaError::CreateValueObject(msg));
        }

        let attributes: Vec<&hcl::Attribute> = block.body.attributes().collect();
        LangTextData::try_from(attributes).map(|lang_text_data| AlternativeName(lang_text_data.0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_try_from_correct_block() {
        let block = hcl::block!(
            alternative_name {
                de = "Der alternative Name"
                en = "The alternative name"
                fr = "Le alternative name"
            }
        );

        let alternative_name = AlternativeName::try_from(&block).unwrap();

        let mut map: HashMap<IsoCode, String> = HashMap::new();
        map.insert(IsoCode::DE, String::from("Der alternative Name"));
        map.insert(IsoCode::EN, String::from("The alternative name"));
        map.insert(IsoCode::FR, String::from("Le alternative name"));
        let expected = AlternativeName(map);

        assert_eq!(alternative_name, expected);
    }

    #[test]
    fn test_try_from_incorrect_block() {
        let block = hcl::block!(
            alternative_name_other {
                de = "Der alternative Name"
                en = "The alternative name"
                fr = "Le alternative name"
            }
        );

        let alternative_name = AlternativeName::try_from(&block);

        assert!(alternative_name.is_err());
    }
}