use serde::Serialize;

use crate::metadata::value::identifier::GrantId;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Grant {
    pub id: GrantId,
}
