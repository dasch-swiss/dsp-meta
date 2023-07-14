pub(crate) mod value_objects;

use std::fmt::Debug;

use serde::{Deserialize, Serialize};

use crate::domain::value_objects::{
    CreatedAt, CreatedBy, Datasets, EndDate, Funders, Grants, HowToCite, Shortcode, StartDate,
    TeaserText, ID,
};

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

#[derive(Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct Project {
    id: ID,
    created_at: CreatedAt,
    created_by: CreatedBy,
    shortcode: Shortcode,
    teaser_text: TeaserText,
    how_to_cite: HowToCite,
    start_date: StartDate,
    end_date: EndDate,
    datasets: Datasets,
    funders: Funders,
    grants: Grants,
}

impl Project {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: ID,
        created_at: CreatedAt,
        created_by: CreatedBy,
        shortcode: Shortcode,
        teaser_text: TeaserText,
        how_to_cite: HowToCite,
        start_date: StartDate,
        end_date: EndDate,
        datasets: Datasets,
        funders: Funders,
        grants: Grants,
    ) -> Self {
        Self {
            id,
            created_at,
            created_by,
            shortcode,
            teaser_text,
            how_to_cite,
            start_date,
            end_date,
            datasets,
            funders,
            grants,
        }
    }
    pub fn id(&self) -> &ID {
        &self.id
    }
    pub fn created_at(&self) -> &CreatedAt {
        &self.created_at
    }
    pub fn created_by(&self) -> &CreatedBy {
        &self.created_by
    }
    pub fn shortcode(&self) -> &Shortcode {
        &self.shortcode
    }
    pub fn teaser_text(&self) -> &TeaserText {
        &self.teaser_text
    }
    pub fn how_to_cite(&self) -> &HowToCite {
        &self.how_to_cite
    }
    pub fn start_date(&self) -> &StartDate {
        &self.start_date
    }
    pub fn end_date(&self) -> &EndDate {
        &self.end_date
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
