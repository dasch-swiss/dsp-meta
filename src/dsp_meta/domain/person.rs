use std::fmt::Debug;

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Person {
    id: String,
}

impl Person {
    pub fn make(id: &str) -> Person {
        Person { id: id.to_string() }
    }
}
