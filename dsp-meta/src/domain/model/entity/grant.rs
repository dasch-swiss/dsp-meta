use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Grant {
    id: String,
}

impl Grant {
    pub fn new(id: &str) -> Self {
        Self { id: id.to_string() }
    }
}
