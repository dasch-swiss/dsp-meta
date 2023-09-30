use serde::Serialize;

pub(crate) mod alternative_name;
pub(crate) mod description;
pub(crate) mod discipline;
pub(crate) mod iso_code;
pub(crate) mod keyword;
mod lang_text_data;
pub(crate) mod publication;
mod ref_data;
mod simple_text_data;
pub(crate) mod spatial_coverage;
pub(crate) mod temporal_coverage;
pub(crate) mod url;
pub(crate) mod version;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct ID(String);

#[derive(Debug, Default, Clone, PartialEq, Serialize)]
pub struct CreatedAt(pub u64);

#[derive(Debug, Default, Clone, PartialEq, Serialize)]
pub struct CreatedBy(pub String);

#[derive(Debug, Default, Clone, PartialEq, Serialize)]
pub struct Shortcode(pub String);

#[derive(Debug, Default, Clone, PartialEq, Serialize)]
pub struct Name(pub String);

#[derive(Debug, Default, Clone, PartialEq, Serialize)]
pub struct TeaserText(pub String);

#[derive(Debug, Default, Clone, PartialEq, Serialize)]
pub struct HowToCite(pub String);

#[derive(Debug, Default, Clone, PartialEq, Serialize)]
pub struct StartDate(pub String);

#[derive(Debug, Default, Clone, PartialEq, Serialize)]
pub struct EndDate(pub String);

#[derive(Debug, Default, Clone, PartialEq, Serialize)]
pub struct ContactPoint(pub String);

#[derive(Debug, Default, Clone, PartialEq, Serialize)]
pub struct Title(pub String);