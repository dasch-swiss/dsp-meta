use std::fmt::Debug;

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Project {
    id: String,
    shortcode: String,
}

impl Project {
    pub fn make(id: &str, shortcode: &str) -> Project {
        Project {
            id: id.to_string(),
            shortcode: shortcode.to_string(),
        }
    }
}
