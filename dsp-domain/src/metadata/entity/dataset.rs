use serde::Serialize;

use crate::metadata::value::identifier::DatasetId;
use crate::metadata::value::{CreatedAt, CreatedBy, Title};

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Dataset {
    pub id: DatasetId,
    pub created_at: CreatedAt,
    pub created_by: CreatedBy,
    pub title: Title,
}
