use dsp_domain::metadata::value::url::Url;
use tracing::warn;

use crate::api::convert::hcl::hcl_block::HclBlock;
use crate::error::DspMetaError;

const URL_BLOCK_IDENTIFIER: &str = "url";
const HREF_ATTRIBUTE_KEY: &str = "href";
const LABEL_ATTRIBUTE_KEY: &str = "label";

impl<'a> TryInto<Url> for HclBlock<'a> {
    type Error = DspMetaError;

    fn try_into(self) -> Result<Url, Self::Error> {
        if self.0.identifier.as_str() != URL_BLOCK_IDENTIFIER {
            let msg = format!(
                "The passed block is not named correctly. Expected '{}', however got '{}' instead.",
                URL_BLOCK_IDENTIFIER,
                self.0.identifier.as_str()
            );
            return Err(DspMetaError::CreateValueObject(msg));
        }

        let mut href: Option<url::Url> = None;
        let mut label: Option<String> = None;

        let attributes: Vec<&hcl::Attribute> = self.0.body.attributes().collect();
        for attribute in attributes {
            match attribute.key() {
                LABEL_ATTRIBUTE_KEY => {
                    if label.is_some() {
                        return Err(DspMetaError::CreateValueObject(
                            "The passed discipline block contains multiple description attributes."
                                .to_string(),
                        ));
                    }
                    label = match attribute.expr() {
                        hcl::Expression::String(value) => Ok(Some(value.to_owned())),
                        _ => Err(DspMetaError::CreateValueObject(
                            "The passed discipline block description attribute is not of String type.".to_string(),
                        )),
                    }?;
                }
                HREF_ATTRIBUTE_KEY => {
                    if href.is_some() {
                        return Err(DspMetaError::CreateValueObject(
                            "Multiple href attributes not allowed.".to_string(),
                        ));
                    }
                    href = match attribute.expr() {
                        hcl::Expression::String(value) => {
                            Ok(Some(url::Url::parse(value).map_err(|_| {
                                DspMetaError::CreateValueObject(
                                    "The passed discipline block url attribute is not a valid url."
                                        .to_string(),
                                )
                            })?))
                        }
                        _ => Err(DspMetaError::CreateValueObject(
                            "The value for the href attribute is not of String type.".to_string(),
                        )),
                    }?;
                }
                _ => {
                    warn!("Parse error: unknown attribute '{}'.", attribute.key());
                }
            }
        }

        Ok(Url {
            href: href.ok_or(DspMetaError::CreateValueObject(
                "The required href attribute is missing.".to_string(),
            ))?,
            label: label.ok_or(DspMetaError::CreateValueObject(
                "The required label attribute is missing.".to_string(),
            ))?,
        })
    }
}

#[cfg(test)]
mod tests {

    use dsp_domain::metadata::value::url::*;

    #[test]
    fn test_try_from_correct_block() {
        let block = hcl::block!(
            url {
                href = "https://www.google.com"
                label = "Google"
            }
        );

        let url = Url::try_from(&block).unwrap();

        let expected = Url {
            href: url::Url::try_from("https://www.google.com").unwrap(),
            label: "Google".to_string(),
        };

        assert_eq!(url, expected);
    }

    #[test]
    fn test_try_from_incorrect_block() {
        let block = hcl::block!(
            url_other {
                href = "https://www.google.com"
                label = "Google"
            }
        );

        let url = Url::try_from(&block);

        assert!(url.is_err());
    }
}
