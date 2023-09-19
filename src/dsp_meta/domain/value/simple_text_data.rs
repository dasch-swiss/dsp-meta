use tracing::warn;

use crate::errors::DspMetaError;

const TEXT_ATTRIBUTE_IDENTIFIER: &str = "text";

#[derive(Debug, PartialEq)]
pub struct SimpleTextData(pub String);

impl TryFrom<Vec<&hcl::Attribute>> for SimpleTextData {
    type Error = DspMetaError;

    fn try_from(attributes: Vec<&hcl::Attribute>) -> Result<Self, Self::Error> {
        let mut text_attribute_value: Option<String> = None;

        for attribute in attributes {
            match attribute.key() {
                TEXT_ATTRIBUTE_IDENTIFIER => {
                    if text_attribute_value.is_some() {
                        return Err(DspMetaError::CreateValueObject(
                            "Multiple text attributes are not allowed.".to_string(),
                        ));
                    }
                    text_attribute_value = match attribute.expr() {
                        hcl::Expression::String(value) => Ok(Some(value.to_owned())),
                        _ => Err(DspMetaError::CreateValueObject(
                            "The attribute value is not of String type.".to_string(),
                        )),
                    }?;
                }
                _ => {
                    warn!("Parse error: unknown attribute '{}'.", attribute.key());
                }
            }
        }
        Ok(SimpleTextData(text_attribute_value.ok_or_else(|| {
            DspMetaError::CreateValueObject("Missing text attribute.".to_string())
        })?))
    }
}

#[cfg(test)]
mod tests {

    use tracing_test::traced_test;

    use super::*;

    #[test]
    fn test_try_from_attributes() {
        let attribute = hcl::Attribute::new("text", "some text");
        let text_data = SimpleTextData::try_from(vec![&attribute]).unwrap();
        assert_eq!(text_data, SimpleTextData("some text".to_string()));
    }

    #[traced_test]
    #[test]
    fn test_try_from_attributes_missing_text() {
        let attribute = hcl::Attribute::new("some_other_attribute", "some text");
        let text_data = SimpleTextData::try_from(vec![&attribute]);
        assert!(text_data.is_err());
        assert!(logs_contain(
            "Parse error: unknown attribute 'some_other_attribute'"
        ));
    }

    #[test]
    fn test_try_from_attributes_multiple_text() {
        let attribute = hcl::Attribute::new("text", "some text");
        let attribute2 = hcl::Attribute::new("text", "some text");
        let text_data = SimpleTextData::try_from(vec![&attribute, &attribute2]);
        assert!(text_data.is_err());
    }

    #[test]
    fn test_try_from_attributes_wrong_type() {
        let attribute = hcl::Attribute::new("text", 1);
        let text_data = SimpleTextData::try_from(vec![&attribute]);
        assert!(text_data.is_err());
    }
}
