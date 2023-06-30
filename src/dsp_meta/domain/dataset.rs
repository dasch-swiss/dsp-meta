use std::fmt::Debug;

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Dataset {
    id: String,
    title: String,
}

impl Dataset {
    pub fn make(id: &str, title: &str) -> Dataset {
        Dataset {
            id: id.to_string(),
            title: title.to_string(),
        }
    }
}
