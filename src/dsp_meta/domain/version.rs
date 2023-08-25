use hcl::{Attribute, Expression};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct Version(pub u64);

/// Given a list of attributes, try to extract the version.
impl TryFrom<Vec<&Attribute>> for Version {
    type Error = crate::errors::DspMetaError;
    fn try_from(attributes: Vec<&Attribute>) -> Result<Self, Self::Error> {
        type Error = crate::errors::DspMetaError;

        let mut result: Result<Self, Error> =
            Err(Error::ParseVersion("Version attribute is not provided."));

        for attribute in attributes {
            if attribute.key() == "version" {
                result = match attribute.expr() {
                    Expression::Number(value) => Ok(Self(value.as_u64().unwrap())),
                    _ => Err(Error::ParseVersion("Version needs to be a number.")),
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use hcl::body;

    use super::*;

    #[test]
    fn test_try_from_attributes() {
        let body = body!(version = 1);

        let attributes: Vec<&hcl::Attribute> = body.attributes().collect();
        let version = Version::try_from(attributes).unwrap();
        assert_eq!(version, Version(1));
    }
}
