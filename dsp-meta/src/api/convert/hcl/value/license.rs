use dsp_domain::metadata::value::license::License;
use tracing::warn;

use crate::api::convert::hcl::hcl_block::HclBlock;
use crate::error::DspMetaError;

const LICENSE_BLOCK_IDENTIFIER: &str = "license";
const TYPE_ATTRIBUTE_KEY: &str = "type";
const HREF_ATTRIBUTE_KEY: &str = "href";
const DATE_ATTRIBUTE_KEY: &str = "date";
const LABEL_ATTRIBUTE_KEY: &str = "label";

impl<'a> TryInto<License> for HclBlock<'a> {
    type Error = DspMetaError;

    fn try_into(self) -> Result<License, Self::Error> {
        if self.0.identifier.as_str() != LICENSE_BLOCK_IDENTIFIER {
            let msg = format!(
                "The passed block is not named correctly. Expected '{}', however got '{}' instead.",
                LICENSE_BLOCK_IDENTIFIER,
                self.0.identifier.as_str()
            );
            return Err(DspMetaError::CreateValueObject(msg));
        }

        let mut type_: Option<String> = None;
        let mut href: Option<url::Url> = None;
        let mut date: Option<String> = None;
        let mut label: Option<String> = None;

        let attributes: Vec<&hcl::Attribute> = self.0.body.attributes().collect();
        for attribute in attributes {
            match attribute.key() {
                TYPE_ATTRIBUTE_KEY => {
                    if type_.is_some() {
                        return Err(DspMetaError::CreateValueObject(
                            format!("The passed {LICENSE_BLOCK_IDENTIFIER} block contains multiple {TYPE_ATTRIBUTE_KEY} attributes.")
                                .to_string(),
                        ));
                    }
                    type_ = match attribute.expr() {
                        hcl::Expression::String(value) => Ok(Some(value.to_owned())),
                        _ => Err(DspMetaError::CreateValueObject(
                            format!("The passed {LICENSE_BLOCK_IDENTIFIER} block {TYPE_ATTRIBUTE_KEY} attribute is not of String type.").to_string(),
                        )),
                    }?;
                }
                HREF_ATTRIBUTE_KEY => {
                    if href.is_some() {
                        return Err(DspMetaError::CreateValueObject(
                            format!("Multiple {TYPE_ATTRIBUTE_KEY} attributes not allowed.")
                                .to_string(),
                        ));
                    }
                    href = match attribute.expr() {
                        hcl::Expression::String(value) => {
                            Ok(Some(url::Url::parse(value).map_err(|_| {
                                DspMetaError::CreateValueObject(
                                    format!("The passed {LICENSE_BLOCK_IDENTIFIER} block {TYPE_ATTRIBUTE_KEY} attribute is not a valid URL.")
                                        .to_string(),
                                )
                            })?))
                        }
                        _ => Err(DspMetaError::CreateValueObject(
                            "The value for the href attribute is not of String type.".to_string(),
                        )),
                    }?;
                }
                DATE_ATTRIBUTE_KEY => {
                    if date.is_some() {
                        return Err(DspMetaError::CreateValueObject(
                            format!("Multiple {DATE_ATTRIBUTE_KEY} attributes not allowed.")
                                .to_string(),
                        ));
                    }
                    date = match attribute.expr() {
                        hcl::Expression::String(value) => Ok(Some(value.to_owned())),
                        _ => Err(DspMetaError::CreateValueObject(
                            format!("The passed {LICENSE_BLOCK_IDENTIFIER} block {DATE_ATTRIBUTE_KEY} attribute is not of String type.").to_string(),
                        )),
                    }?;
                }
                LABEL_ATTRIBUTE_KEY => {
                    if label.is_some() {
                        return Err(DspMetaError::CreateValueObject(
                            format!("The passed {LICENSE_BLOCK_IDENTIFIER} block contains multiple {LABEL_ATTRIBUTE_KEY} attributes.")
                                .to_string(),
                        ));
                    }
                    label = match attribute.expr() {
                        hcl::Expression::String(value) => Ok(Some(value.to_owned())),
                        _ => Err(DspMetaError::CreateValueObject(
                            format!("The passed {LICENSE_BLOCK_IDENTIFIER} block {LABEL_ATTRIBUTE_KEY} attribute is not of String type.").to_string(),
                        )),
                    }?;
                }
                _ => {
                    warn!("Parse error: unknown attribute '{}'.", attribute.key());
                }
            }
        }

        Ok(License {
            license_type: type_.ok_or_else(|| {
                DspMetaError::CreateDomainObject(
                    "The required href attribute is missing.".to_string(),
                )
            })?,
            href: href.ok_or_else(|| {
                DspMetaError::CreateDomainObject(
                    "The required href attribute is missing.".to_string(),
                )
            })?,
            date: date.ok_or_else(|| {
                DspMetaError::CreateDomainObject(
                    "The required date attribute is missing.".to_string(),
                )
            })?,
            label: label.ok_or_else(|| {
                DspMetaError::CreateDomainObject(
                    "The required label attribute is missing.".to_string(),
                )
            })?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::convert::hcl::hcl_block::HclBlock;
    use crate::error::DspMetaError;

    #[test]
    fn test_try_from_correct_block() {
        let block = hcl::block!(
            license {
                type = "creative_commons"
                href = "https://www.google.com"
                date  = "2021-09-02"
                label = "CC BY-NC 4.0"
            }
        );

        let result: License = HclBlock(&block).try_into().unwrap();

        let expected = License {
            license_type: "creative_commons".to_string(),
            href: url::Url::try_from("https://www.google.com").unwrap(),
            date: "2021-09-02".to_string(),
            label: "CC BY-NC 4.0".to_string(),
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_try_from_incorrect_block() {
        let block = hcl::block!(
            other_block {
                type = "creative_commons"
                href = "https://www.google.com"
                date  = "2021-09-02"
                label = "CC BY-NC 4.0"
            }
        );

        let result: Result<License, DspMetaError> = HclBlock(&block).try_into();

        assert!(result.is_err());
    }
}
