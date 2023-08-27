use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Person {
    id: String,
}

impl From<&str> for Person {
    fn from(id: &str) -> Self {
        Self { id: id.to_string() }
    }
}
