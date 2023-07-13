use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub enum ProjectValues {
    ID(ID),
    CreatedAt(CreatedAt),
    CreatedBy(CreatedBy),
    Shortcode(Shortcode),
    TeaserText(TeaserText),
    HowToCite(HowToCite),
    StartDate(StartDate),
    EndDate(EndDate),
    Datasets(Datasets),
    Funders(Funders),
    Grants(Grants),
}

#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
pub struct ID(String);

impl ID {
    pub fn new(id: &str) -> Self {
        Self(id.to_string())
    }
    pub fn value(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
pub struct CreatedAt(u64);
impl CreatedAt {
    pub fn new(created_at: u64) -> Self {
        Self(created_at)
    }
    pub fn value(&self) -> u64 {
        self.0
    }
}

#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
pub struct CreatedBy(String);
impl CreatedBy {
    pub fn new(created_by: &str) -> Self {
        Self(created_by.to_string())
    }
    pub fn value(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
pub struct Shortcode(String);
impl Shortcode {
    pub fn new(shortcode: &str) -> Self {
        Self(shortcode.to_string())
    }
    pub fn value(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
pub struct TeaserText(String);
impl TeaserText {
    pub fn new(teaser_text: &str) -> Self {
        Self(teaser_text.to_string())
    }
    pub fn value(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
pub struct HowToCite(String);
impl HowToCite {
    pub fn new(how_to_cite: &str) -> Self {
        Self(how_to_cite.to_string())
    }
    pub fn value(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
pub struct StartDate(String);
impl StartDate {
    pub fn new(start_date: &str) -> Self {
        Self(start_date.to_string())
    }
    pub fn value(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
pub struct EndDate(String);
impl EndDate {
    pub fn new(end_date: &str) -> Self {
        Self(end_date.to_string())
    }
    pub fn value(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
pub struct Datasets(Vec<String>);
impl Datasets {
    pub fn new(datasets: Vec<String>) -> Self {
        Self(datasets)
    }
    pub fn value(&self) -> Vec<String> {
        self.0.clone()
    }
}

#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
pub struct Funders(Vec<String>);
impl Funders {
    pub fn new(funders: Vec<String>) -> Self {
        Self(funders)
    }
    pub fn value(&self) -> Vec<String> {
        self.0.clone()
    }
}

#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
pub struct Grants(Vec<String>);
impl Grants {
    pub fn new(grants: Vec<String>) -> Self {
        Self(grants)
    }
    pub fn value(&self) -> Vec<String> {
        self.0.clone()
    }
}
