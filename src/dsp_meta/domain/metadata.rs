use std::fmt::Debug;

use serde::{Deserialize, Serialize};

use crate::domain::dataset::Dataset;
use crate::domain::person::Person;
use crate::domain::project::Project;

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Metadata {
    project: Project,
    datasets: Vec<Dataset>,
    persons: Vec<Person>,
}

impl Metadata {
    pub fn make(project: Project, datasets: Vec<Dataset>, persons: Vec<Person>) -> Metadata {
        Metadata {
            project: project,
            datasets: datasets,
            persons: persons,
        }
    }
}
