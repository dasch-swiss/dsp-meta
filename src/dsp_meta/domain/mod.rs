pub(crate) mod project;
pub(crate) mod value_objects;

use std::fmt::Debug;

use project::Project;
use serde::{Deserialize, Serialize};

// move everything to domain module

#[derive(Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct Metadata {
    version: u64,
    project: Project,
    datasets: Vec<Dataset>,
    grants: Vec<Grant>,
    organizations: Vec<Organization>,
    persons: Vec<Person>,
}

impl Metadata {
    pub fn new(
        version: u64,
        project: Project,
        datasets: Vec<Dataset>,
        grants: Vec<Grant>,
        organizations: Vec<Organization>,
        persons: Vec<Person>,
    ) -> Self {
        Self {
            version,
            project,
            datasets,
            grants,
            organizations,
            persons,
        }
    }
    pub fn version(&self) -> u64 {
        self.version
    }
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Dataset {
    id: String,
    title: String,
}

impl Dataset {
    pub fn new(id: &str, title: &str) -> Self {
        Self {
            id: id.to_string(),
            title: title.to_string(),
        }
    }
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Person {
    id: String,
}

impl Person {
    pub fn new(id: &str) -> Self {
        Self { id: id.to_string() }
    }
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Organization {
    id: String,
}

impl Organization {
    pub fn new(id: &str) -> Self {
        Self { id: id.to_string() }
    }
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Grant {
    id: String,
}

impl Grant {
    pub fn new(id: &str) -> Self {
        Self { id: id.to_string() }
    }
}
