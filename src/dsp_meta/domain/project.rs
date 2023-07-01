use std::fmt::Debug;

use serde::{Deserialize, Serialize};

use crate::domain::dataset::Dataset;
use crate::domain::person::Person;

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Project {
    id: String,
    shortcode: String,
    datasets: Vec<Dataset>,
    persons: Vec<Person>,
}

impl Project {
    pub fn make(
        id: &str,
        shortcode: &str,
        datasets: Vec<Dataset>,
        persons: Vec<Person>,
    ) -> Project {
        Project {
            id: id.to_string(),
            shortcode: shortcode.to_string(),
            datasets,
            persons,
        }
    }
}
