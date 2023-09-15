use tracing::warn;

use crate::errors::DspMetaError;

#[derive(Debug, PartialEq)]
pub struct RefData {
    pub(crate) ref_id: String,
    pub(crate) description: String,
    pub(crate) url: url::Url,
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
