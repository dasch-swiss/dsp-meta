use serde::Serialize;

use crate::metadata::value::identifier::AgentId;

/// Represents an HCL block which consists of attribute keys and a value expressions.
///
/// In HCL syntax this is represented as:
///
/// ```hcl
/// attribution {
///   agent = "identifier of the agent"
///   roles = ["role1", "role2"]
/// }
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct Attribution {
    pub agent: AgentId,
    pub roles: Vec<String>,
}

impl Default for Attribution {
    fn default() -> Self {
        Attribution {
            agent: AgentId("Default agent ID".to_string()),
            roles: vec!["role1".to_string(), "role2".to_string()],
        }
    }
}
