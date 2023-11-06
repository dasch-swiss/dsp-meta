use std::collections::HashMap;

use serde::Serialize;

use crate::domain::model::value::iso_code::IsoCode;
use crate::domain::model::value::lang_text_data::LangTextData;
use crate::error::DspMetaError;

const DESCRIPTION_BLOCK_IDENTIFIER: &str = "description";

/// A set of descriptions in different languages.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Description(HashMap<IsoCode, String>);

impl Default for Description {
    fn default() -> Self {
        let mut map: HashMap<IsoCode, String> = HashMap::new();
        map.insert(IsoCode::DE, String::from("Die Default-Beschreibung."));
        map.insert(IsoCode::EN, String::from("The default description."));
        map.insert(IsoCode::FR, String::from("Le standard description."));
        Self(map)
    }
}

impl TryFrom<&hcl::Block> for Description {
    type Error = DspMetaError;

    fn try_from(block: &hcl::Block) -> Result<Self, Self::Error> {
        if block.identifier.as_str() != DESCRIPTION_BLOCK_IDENTIFIER {
            let msg = format!(
                "The passed block is not named correctly. Expected '{}', however got '{}' instead.",
                DESCRIPTION_BLOCK_IDENTIFIER,
                block.identifier.as_str()
            );
            return Err(DspMetaError::CreateValueObject(msg));
        }

        let attributes: Vec<&hcl::Attribute> = block.body.attributes().collect();
        LangTextData::try_from(attributes).map(|lang_text_data| Description(lang_text_data.0))
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_try_from_correct_block() {
        let block = hcl::block!(
            description {
                de = "Die Beschreibung"
                en = "The description"
                fr = "La description"
            }
        );

        let description = Description::try_from(&block).unwrap();

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

        let description = Description::try_from(&block);

        assert!(description.is_err());
    }
}
