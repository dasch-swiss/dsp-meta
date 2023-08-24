use hcl::{Attribute, Expression};

use crate::errors::DspMetaError;

pub fn parse_version(attributes: Vec<&Attribute>) -> Result<u64, DspMetaError> {
    let mut version: u64 = 0;
    for attribute in attributes {
        if attribute.key() == "version" {
            version = match attribute.expr() {
                Expression::Number(value) => Ok(value.as_u64().unwrap()),
                _ => Err(DspMetaError::ParseVersion("Version needs to be a number.")),
            }?
        }
    }
    Ok(version)
}

#[cfg(test)]
mod tests {
    #[test]
    fn parse_version() {
        let input = r#"
            version = 1
        "#;

        let body: hcl::Body = hcl::from_str(input).unwrap();
        let attributes: Vec<&hcl::Attribute> = body.attributes().collect();
        let version = super::parse_version(attributes).unwrap();
        assert_eq!(version, 1);
    }
}
