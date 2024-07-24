use nonempty::NonEmpty;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// This model corresponds to the json schema found in /data/schema-metadata-draft.json
// These data structures are able to parse all json metadata found in /data/json/.*json
// We can use them to produce TOML or YAML files as well

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DraftMetadata {
    pub project: DraftProject,
    pub datasets: Option<Vec<DraftDataset>>,
    pub persons: Option<Vec<DraftPerson>>,
    pub organizations: Option<Vec<DraftOrganization>>,
    pub grants: Option<Vec<DraftGrant>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DraftProject {
    #[serde(rename = "__id")]
    pub id: String,
    #[serde(rename = "__createdAt")]
    pub created_at: Option<String>,
    #[serde(rename = "__createdBy")]
    pub created_by: Option<String>,
    pub shortcode: String,
    pub name: String,
    pub description: Option<DraftText>,
    pub start_date: DraftDate,
    pub teaser_text: String,
    pub datasets: NonEmpty<String>,
    pub keywords: NonEmpty<DraftText>,
    pub disciplines: NonEmpty<DraftTextOrUrl>,
    pub temporal_coverage: Option<NonEmpty<DraftTextOrUrl>>,
    pub spatial_coverage: Option<NonEmpty<DraftUrl>>,
    pub funders: Option<NonEmpty<String>>,
    pub url: Option<DraftUrl>,
    pub secondary_url: Option<DraftUrl>,
    pub data_management_plan: Option<DraftDataManagementPlan>,
    pub end_date: Option<DraftDate>,
    pub contact_point: Option<String>,
    pub how_to_cite: Option<String>,
    pub publications: Option<NonEmpty<DraftPublication>>,
    pub grants: Option<NonEmpty<String>>,
    pub alternative_names: Option<NonEmpty<DraftText>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct DraftPublication {
    pub text: String,
    pub url: Option<NonEmpty<DraftUrl>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DraftDataset {
    #[serde(rename = "__id")]
    pub id: String,
    #[serde(rename = "__createdAt")]
    pub created_at: Option<String>,
    #[serde(rename = "__createdBy")]
    pub created_by: Option<String>,
    pub abstracts: Option<NonEmpty<DraftTextOrUrl>>,
    pub access_conditions: Option<DraftAccessCondition>,
    pub additional: Option<Vec<DraftTextOrUrl>>,
    pub alternative_titles: Option<Vec<DraftText>>,
    pub attributions: Option<NonEmpty<DraftAttribution>>,
    pub date_created: Option<DraftDate>,
    pub date_modified: Option<DraftDate>,
    pub date_published: Option<DraftDate>,
    pub distribution: Option<DraftUrl>,
    pub how_to_cite: Option<String>,
    pub languages: Option<NonEmpty<DraftText>>,
    pub licenses: Option<NonEmpty<DraftLicense>>,
    pub status: Option<DraftStatus>,
    pub title: Option<String>,
    pub type_of_data: Option<NonEmpty<DraftTypeOfData>>,
    pub urls: Option<Vec<DraftUrl>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum DraftAccessCondition {
    Open,
    Restricted,
    Closed,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum DraftStatus {
    #[serde(rename = "In planning")]
    InPlanning,
    #[serde(rename = "Ongoing")]
    OnGoing,
    #[serde(rename = "On hold")]
    OnHold,
    Finished,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum DraftTypeOfData {
    XML,
    Text,
    Image,
    Video,
    Audio,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DraftPerson {
    #[serde(rename = "__id")]
    pub id: String,
    #[serde(rename = "__createdAt")]
    pub created_at: Option<String>,
    #[serde(rename = "__createdBy")]
    pub created_by: Option<String>,
    pub job_titles: Option<NonEmpty<String>>,
    pub given_names: NonEmpty<String>,
    pub family_names: NonEmpty<String>,
    pub affiliation: Option<NonEmpty<String>>,
    pub address: Option<DraftAddress>,
    pub email: Option<String>,
    pub secondary_email: Option<String>,
    pub authority_refs: Option<Vec<DraftUrl>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DraftOrganization {
    #[serde(rename = "__id")]
    pub id: String,
    #[serde(rename = "__createdAt")]
    pub created_at: Option<String>,
    #[serde(rename = "__createdBy")]
    pub created_by: Option<String>,
    pub name: String,
    pub url: Option<DraftUrl>,
    pub address: Option<DraftAddress>,
    pub email: Option<String>,
    pub alternative_names: Option<Vec<DraftText>>,
    pub authority_refs: Option<Vec<DraftUrl>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DraftGrant {
    #[serde(rename = "__id")]
    pub id: String,
    #[serde(rename = "__createdAt")]
    pub created_at: Option<String>,
    #[serde(rename = "__createdBy")]
    pub created_by: Option<String>,
    pub funders: NonEmpty<String>,
    pub number: Option<String>,
    pub name: Option<String>,
    pub url: Option<DraftUrl>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct DraftText(HashMap<DraftIsoCode, String>);

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub struct DraftIsoCode(String);

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct DraftDate(String);

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DraftUrl {
    pub url: String,
    pub text: Option<String>,
    #[serde(rename = "type")]
    #[serde(default = "DraftUrlType::default")]
    pub url_type: DraftUrlType,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum DraftUrlType {
    URL,
    Geonames,
    Pleiades,
    Skos,
    Periodo,
    Chronontology,
    GND,
    VIAF,
    Grid,
    ORCID,
    #[serde(rename = "Creative Commons")]
    CreativeCommons,
    DOI,
    ARK,
}
impl DraftUrlType {
    fn default() -> Self {
        DraftUrlType::URL
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DraftAddress {
    pub street: String,
    pub postal_code: String,
    pub locality: Option<String>,
    pub country: String,
    pub canton: Option<String>,
    pub additional: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DraftDataManagementPlan {
    pub available: bool,
    pub url: Option<DraftUrl>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DraftAttribution {
    pub agent: String,
    pub roles: NonEmpty<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DraftLicense {
    pub license: DraftUrl,
    pub date: DraftDate,
    pub details: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum DraftTextOrUrl {
    TextValue(DraftText),
    UrlValue(DraftUrl),
}
