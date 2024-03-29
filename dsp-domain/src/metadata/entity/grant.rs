use serde::Serialize;

use crate::metadata::value::funder::Funder;
use crate::metadata::value::identifier::GrantId;
use crate::metadata::value::url::Url;
use crate::metadata::value::{CreatedAt, CreatedBy, GrantNumber, GrantType, Name};

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Grant {
    pub id: GrantId,                 // (1)
    pub created_at: CreatedAt,       // (1)
    pub created_by: CreatedBy,       // (1)
    pub type_of_grant: GrantType,    // (1)
    pub name: Option<Name>,          // (0-1)
    pub number: Option<GrantNumber>, // (0-1)
    pub url: Option<Url>,            // (0-1)
    pub funders: Vec<Funder>,        // (1-n)
}
