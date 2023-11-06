use dsp_domain::metadata::value::version::Version;
use hcl::Expression;

/// Given a list of attributes, try to extract the version.
impl TryFrom<&hcl::Attribute> for Version {
    type Error = crate::error::DspMetaError;
    fn try_from(attribute: &hcl::Attribute) -> Result<Self, Self::Error> {
        type Error = crate::error::DspMetaError;

        let mut result: Result<Self, Error> = Err(Error::ParseVersion(
            "Version attribute is not provided.".to_string(),
        ));

        if attribute.key() == "version" {
            result = match attribute.expr() {
                Expression::Number(value) => Ok(Self(value.as_u64().ok_or_else(|| {
                    Error::ParseVersion("Version needs to be a non-negative number.".to_string())
                })?)),
                _ => Err(Error::ParseVersion(
                    "Version needs to be a non-negative number.".to_string(),
                )),
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use dsp_domain::metadata::value::version::*;

    #[test]
    fn test_try_from_attributes() {
        let attribute = hcl::Attribute::new("version", 1u64);
        let version = Version::try_from(&attribute).unwrap();
        assert_eq!(version, Version(1));
    }
}
