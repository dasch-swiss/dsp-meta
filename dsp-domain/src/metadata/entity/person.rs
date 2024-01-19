use serde::Serialize;

use crate::metadata::value::identifier::PersonId;

#[derive(Debug, Clone, Default, PartialEq, Serialize)]
pub struct Person {
    pub id: PersonId,
}
