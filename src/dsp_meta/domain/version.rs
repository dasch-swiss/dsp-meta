use hcl::{Attribute, Expression};

#[derive(Debug, PartialEq)]
pub struct Version(pub u64);

/// Given a list of attributes, try to extract the version.
impl TryFrom<&Attribute> for Version {
    type Error = crate::errors::DspMetaError;
    fn try_from(attribute: &Attribute) -> Result<Self, Self::Error> {
        type Error = crate::errors::DspMetaError;

        let mut result: Result<Self, Error> =
            Err(Error::ParseVersion("Version attribute is not provided."));

        if attribute.key() == "version" {
            result = match attribute.expr() {
                Expression::Number(value) => Ok(Self(value.as_u64().unwrap())),
                _ => Err(Error::ParseVersion("Version needs to be a number.")),
            }
        }

        result
    }
}

impl<'a> Default for &'a Version {
    fn default() -> Self {
        &Version(1)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_try_from_attributes() {
        let attribute = Attribute::new("version", 1u64);
        let version = Version::try_from(&attribute).unwrap();
        assert_eq!(version, Version(1));
    }
}
