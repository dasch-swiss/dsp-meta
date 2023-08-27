use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Dataset {
    pub id: String,
    pub title: String,
}
