use std::collections::HashMap;

use tracing::warn;

use crate::domain::value::iso_code::IsoCode;
use crate::errors::DspMetaError;

/// The discipline of a project can be defined in two ways:
/// 1. A reference to a discipline defined in an external reference system (e.g. SNF or SKOS)
/// 2. A text description of the discipline
///
/// Example:
/// ```hcl
/// discipline skos {
///     ref_id = "https://skos.um.es/unesco6/5501"
///     description = "Local history"
///     url = "https://skos.um.es/unesco6/5501"
/// }
/// ```
#[derive(Debug, PartialEq)]
pub enum Discipline {
    Skos(RefData),
    Snf(RefData),
    Text(TextData),
}

impl TryFrom<&hcl::Block> for Discipline {
    type Error = DspMetaError;

    fn try_from(block: &hcl::Block) -> Result<Self, Self::Error> {
        if block.identifier.as_str() != "discipline" {
            let msg = format!(
                "The passed block is not named correctly. Expected 'discipline', however got '{}' instead.",
                block.identifier.as_str()
            );
            return Err(DspMetaError::CreateValueObject(msg));
        }

        if block.labels.len() != 1 {
            return Err(DspMetaError::CreateValueObject("The passed number of block labels is not correct. Expected '1', namely 'reference data type' (e.g., 'skos').".to_string()));
        }

        let reference_data_type = block.labels.first().ok_or_else(|| {
            DspMetaError::CreateValueObject(
                "The passed discipline block is missing the reference data type label.".to_string(),
            )
        })?;

        let attributes: Vec<&hcl::Attribute> = block.body.attributes().collect();

        match reference_data_type.as_str() {
            "skos" => {
                let ref_data = RefData::try_from(attributes)?;
                Ok(Discipline::Skos(ref_data))
            }
            "snf" => {
                let ref_data = RefData::try_from(attributes)?;
                Ok(Discipline::Snf(ref_data))
            }
            "text" => {
                let text_data = TextData::try_from(attributes)?;
                Ok(Discipline::Text(text_data))
            }
            _ => {
                Err(DspMetaError::CreateValueObject("The passed discipline block is missing the correct reference data type label: 'skos', 'snf', or 'text'.".to_string()))
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct RefData {
    ref_id: String,
    description: String,
    url: url::Url,
}

/// Reference to a discipline defined in an external reference system (e.g. SNF or SKOS)
/// FIXME: Move to the API layer where the service adapter is implemented
impl TryFrom<Vec<&hcl::Attribute>> for RefData {
    type Error = DspMetaError;

    fn try_from(attributes: Vec<&hcl::Attribute>) -> Result<Self, Self::Error> {
        let mut ref_id: Option<String> = None;
        let mut description: Option<String> = None;
        let mut url: Option<url::Url> = None;

        for attribute in attributes {
            match attribute.key() {
                "ref_id" => {
                    if ref_id.is_some() {
                        return Err(DspMetaError::CreateValueObject(
                            "The passed discipline block contains multiple ref_id attributes."
                                .to_string(),
                        ));
                    }
                    ref_id = match attribute.expr() {
                        hcl::Expression::String(value) => Ok(Some(value.to_owned())),
                        _ => Err(DspMetaError::CreateValueObject(
                            "The passed discipline block ref_id attribute is not of String type."
                                .to_string(),
                        )),
                    }?;
                }
                "description" => {
                    if description.is_some() {
                        return Err(DspMetaError::CreateValueObject(
                            "The passed discipline block contains multiple description attributes."
                                .to_string(),
                        ));
                    }
                    description = match attribute.expr() {
                        hcl::Expression::String(value) => Ok(Some(value.to_owned())),
                        _ => Err(DspMetaError::CreateValueObject(
                            "The passed discipline block description attribute is not of String type.".to_string(),
                        )),
                    }?;
                }
                "url" => {
                    if url.is_some() {
                        return Err(DspMetaError::CreateValueObject(
                            "The passed discipline block contains multiple url attributes."
                                .to_string(),
                        ));
                    }
                    url = match attribute.expr() {
                        hcl::Expression::String(value) => {
                            Ok(Some(url::Url::parse(value).map_err(|_| {
                                DspMetaError::CreateValueObject(
                                    "The passed discipline block url attribute is not a valid url."
                                        .to_string(),
                                )
                            })?))
                        }
                        _ => Err(DspMetaError::CreateValueObject(
                            "The passed discipline block url attribute is not of String type."
                                .to_string(),
                        )),
                    }?;
                }
                _ => {
                    warn!("Parse error: unknown attribute '{}'.", attribute.key());
                }
            }
        }

        Ok(RefData {
            ref_id: ref_id.ok_or(DspMetaError::CreateValueObject(
                "The passed discipline block does not contain a ref_id attribute.".to_string(),
            ))?,
            description: description.ok_or(DspMetaError::CreateValueObject(
                "The passed discipline block does not contain a description attribute.".to_string(),
            ))?,
            url: url.ok_or(DspMetaError::CreateValueObject(
                "The passed discipline block does not contain a url attribute.".to_string(),
            ))?,
        })
    }
}

#[derive(Debug, PartialEq)]
pub struct TextData(HashMap<IsoCode, String>);

/// Try to create the text description of the discipline
/// FIXME: Move to the API layer where the service adapter is implemented
impl TryFrom<Vec<&hcl::Attribute>> for TextData {
    type Error = DspMetaError;

    fn try_from(attributes: Vec<&hcl::Attribute>) -> Result<Self, Self::Error> {
        let mut text_data: HashMap<IsoCode, String> = HashMap::new();

        for attribute in attributes {
            let iso_code = IsoCode::try_from(attribute.key())?;
            let text = match attribute.expr() {
                hcl::Expression::String(value) => Ok(value.to_owned()),
                _ => Err(DspMetaError::CreateValueObject(
                    "The passed discipline block description attribute is not of String type."
                        .to_string(),
                )),
            }?;

            text_data.insert(iso_code, text);
        }

        Ok(TextData(text_data))
    }
}

#[cfg(test)]
mod tests {

    use tracing_test::traced_test;

    use super::*;

    #[test]
    #[traced_test]
    fn test_try_from_block_with_skos() {
        let block = hcl::block!(
            discipline skos {
                    ref_id = "https://skos.um.es/unesco6/5501"
                    description = "Local history"
                    url = "https://skos.um.es/unesco6/5501"
            }
        );

        let input = Discipline::try_from(&block).unwrap();
        let expected = Discipline::Skos(RefData {
            ref_id: "https://skos.um.es/unesco6/5501".to_string(),
            description: "Local history".to_string(),
            url: url::Url::parse("https://skos.um.es/unesco6/5501").unwrap(),
        });

        assert_eq!(input, expected);
    }

    #[test]
    #[traced_test]
    fn test_try_from_block_with_text() {
        let block = hcl::block!(
            discipline text {
                    de = "Lokalgeschichte"
                    en = "Local history"
                    fr = "Histoire locale"
            }
        );

        let input = Discipline::try_from(&block).unwrap();
        let expected = Discipline::Text(TextData(
            vec![
                (IsoCode::DE, "Lokalgeschichte".to_string()),
                (IsoCode::EN, "Local history".to_string()),
                (IsoCode::FR, "Histoire locale".to_string()),
            ]
            .into_iter()
            .collect(),
        ));

        assert_eq!(input, expected);
    }
}
