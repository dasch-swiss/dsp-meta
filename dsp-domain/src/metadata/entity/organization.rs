use serde::Serialize;

use crate::metadata::value::identifier::OrganizationId;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Organization {
    pub id: OrganizationId,
}
