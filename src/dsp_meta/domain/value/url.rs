use tracing::warn;

use crate::errors::DspMetaError;

const URL_BLOCK_IDENTIFIER: &str = "url";
const HREF_ATTRIBUTE_KEY: &str = "href";
const LABEL_ATTRIBUTE_KEY: &str = "label";

/// Represents an HCL attribute which consists of an attribute key and a value expression.
///
/// In HCL syntax this is represented as:
///
/// ```hcl
/// url {
///   href = "https://www.google.com"
///   label = "text describing the link"
/// }
/// ```
///
/// Use [`Attribute::new`] to construct an [`Attribute`] from a value that is convertible to this
/// crate's [`Expression`] type.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Url {
    pub href: url::Url,
    pub label: String,
}

impl Url {
    pub fn new(url_string: String, label: String) -> Result<Self, DspMetaError> {
        let maybe_url = url::Url::try_from(url_string.as_str());
        match maybe_url {
            Ok(href) => Ok(Url { href, label }),
            Err(_) => Err(DspMetaError::CreateValueObject(
                "Creating an UrlValue failed because provided value is not a valid URL."
                    .to_string(),
            )),
        }
    }
}

impl Default for Url {
    fn default() -> Self {
        Url {
            href: url::Url::try_from("https://default.xyz").unwrap(),
            label: "Default URL description".to_string(),
        }
    }
}

impl TryFrom<&hcl::Block> for Url {
    type Error = DspMetaError;

    fn try_from(block: &hcl::Block) -> Result<Self, Self::Error> {
        if block.identifier.as_str() != URL_BLOCK_IDENTIFIER {
            let msg = format!(
                "The passed block is not named correctly. Expected '{}', however got '{}' instead.",
                URL_BLOCK_IDENTIFIER,
                block.identifier.as_str()
            );
            return Err(DspMetaError::CreateValueObject(msg));
        }

        let mut href: Option<url::Url> = None;
        let mut label: Option<String> = None;

        let attributes: Vec<&hcl::Attribute> = block.body.attributes().collect();
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

    use super::*;

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
