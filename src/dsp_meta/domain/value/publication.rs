use crate::domain::value::simple_text_data::SimpleTextData;
use crate::errors::DspMetaError;

const PUBLICATION_BLOCK_IDENTIFIER: &str = "publication";

#[derive(Clone, Debug, PartialEq)]
pub enum Publication {
    SimpleText(SimpleTextData),
}

impl TryFrom<&hcl::Block> for Publication {
    type Error = DspMetaError;

    fn try_from(block: &hcl::Block) -> Result<Self, Self::Error> {
        if block.identifier.as_str() != PUBLICATION_BLOCK_IDENTIFIER {
            let msg = format!(
                "The passed block is not named correctly. Expected 'publication', however got '{}' instead.",
                block.identifier.as_str()
            );
            return Err(DspMetaError::CreateValueObject(msg));
        }

        let attributes: Vec<&hcl::Attribute> = block.body.attributes().collect();

        SimpleTextData::try_from(attributes).map(Publication::SimpleText)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_try_from_correct_block() {
        let block = hcl::block!(
            publication {
                text = "A publication"
            }
        );

        let publication = Publication::try_from(&block).unwrap();

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

        let publication = Publication::try_from(&block);

        assert!(publication.is_err());
    }
}
