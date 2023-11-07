use dsp_domain::metadata::value::version::Version;
use hcl::Expression;

use crate::api::convert::hcl::hcl_attribute::HclAttribute;
use crate::error::DspMetaError;

/// Given a list of attributes, try to extract the version.
impl<'a> TryInto<Version> for HclAttribute<'a> {
    type Error = DspMetaError;

    fn try_into(self) -> Result<Version, Self::Error> {
        type Error = crate::error::DspMetaError;

        let mut result: Result<Version, Self::Error> = Err(Error::ParseVersion(
            "Version attribute is not provided.".to_string(),
        ));

        if self.0.key() == "version" {
            result = match self.0.expr() {
                Expression::Number(value) => Ok(Version(value.as_u64().ok_or_else(|| {
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

    use crate::api::convert::hcl::hcl_attribute::HclAttribute;

    #[test]
    fn test_try_from_attributes() {
        let attribute = hcl::Attribute::new("version", 1u64);
        let version: Version = HclAttribute(&attribute).try_into().unwrap();
        assert_eq!(version, Version(1));
    }
}
