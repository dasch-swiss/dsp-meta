use dsp_domain::metadata::value::attribution::Attribution;
use dsp_domain::metadata::value::identifier::AgentId;
use dsp_domain::metadata::value::license::License;
use tracing::warn;

use crate::api::convert::hcl::hcl_block::HclBlock;
use crate::error::DspMetaError;

const BLOCK_IDENTIFIER: &str = "attribution";
const AGENT_ATTRIBUTE_KEY: &str = "agent";
const ROLES_ATTRIBUTE_KEY: &str = "roles";

impl<'a> TryInto<Attribution> for HclBlock<'a> {
    type Error = DspMetaError;

    fn try_into(self) -> Result<Attribution, Self::Error> {
        if self.0.identifier.as_str() != BLOCK_IDENTIFIER {
            let msg = format!(
                "The passed block is not named correctly. Expected '{}', however got '{}' instead.",
                BLOCK_IDENTIFIER,
                self.0.identifier.as_str()
            );
            return Err(DspMetaError::CreateValueObject(msg));
        }

        let mut maybe_agent: Option<AgentId> = None;
        let mut maybe_roles: Vec<String> = vec![];

        let attributes: Vec<&hcl::Attribute> = self.0.body.attributes().collect();
        for attribute in attributes {
            match attribute.key() {
                AGENT_ATTRIBUTE_KEY => {
                    if maybe_agent.is_some() {
                        return Err(DspMetaError::CreateValueObject(
                            format!("The passed {BLOCK_IDENTIFIER} block contains multiple {AGENT_ATTRIBUTE_KEY} attributes.")
                                .to_string(),
                        ));
                    }
                    maybe_agent = match attribute.expr() {
                        hcl::Expression::String(value) => Ok(Some(AgentId(value.to_owned()))),
                        _ => Err(DspMetaError::CreateValueObject(
                            format!("The passed {BLOCK_IDENTIFIER} block {AGENT_ATTRIBUTE_KEY} attribute is not of String type.").to_string(),
                        )),
                    }?;
                },
                ROLES_ATTRIBUTE_KEY => {
                    match attribute.expr() {
                        hcl::Expression::Array(values) => {
                            for value in values {
                                match value {
                                    hcl::Expression::String(value) => maybe_roles.push(value.to_owned()),
                                    _ => {
                                        return Err(DspMetaError::CreateValueObject(
                                            format!("The passed {BLOCK_IDENTIFIER} block {ROLES_ATTRIBUTE_KEY} attribute is not of String type.").to_string(),
                                        ))
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(DspMetaError::CreateValueObject(
                                format!("The passed {BLOCK_IDENTIFIER} block {ROLES_ATTRIBUTE_KEY} attribute is not of List type.").to_string(),
                            ))
                        }
                    }
                }
                _ => {
                    warn!("Parse error: unknown attribute '{}'.", attribute.key());
                }
            }
        }

        let agent = maybe_agent.ok_or(
            DspMetaError::CreateValueObject(
                format!("The passed {BLOCK_IDENTIFIER} block does not contain a {AGENT_ATTRIBUTE_KEY} attribute.")
                    .to_string(),
            )
        )?;

        let roles = if !maybe_roles.is_empty() {
            Ok(maybe_roles)
        } else {
            Err(DspMetaError::CreateValueObject(
                format!("The passed {BLOCK_IDENTIFIER} block does not contain a {ROLES_ATTRIBUTE_KEY} attribute.")
                    .to_string(),
            ))
        }?;

        Ok(Attribution { agent, roles })
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
            attribution {
                agent = "http://ns.dasch.swiss/repository#dsp-081C-organization-000" // reference to person or organization
                roles = ["Author"]
            }
        );

        let result: Attribution = HclBlock(&block).try_into().unwrap();

        let expected = Attribution {
            agent: AgentId(String::from(
                "http://ns.dasch.swiss/repository#dsp-081C-organization-000",
            )),
            roles: vec![String::from("Author")],
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_try_from_incorrect_block() {
        let block = hcl::block!(
            other_block {
                agent = "http://ns.dasch.swiss/repository#dsp-081C-organization-000" // reference to person or organization
                roles = ["Author"]
            }
        );

        let result: Result<License, DspMetaError> = HclBlock(&block).try_into();

        assert!(result.is_err());
    }
}
