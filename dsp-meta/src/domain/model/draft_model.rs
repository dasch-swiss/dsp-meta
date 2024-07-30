use std::collections::HashMap;

use chrono::NaiveDate;
use nonempty::NonEmpty;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

// This model corresponds to the json schema found in resources/schema-metadata-draft.json
// These data structures are able to parse all json metadata found in /data/json/.*json
// We can use them to produce TOML or YAML files as well

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DraftMetadata {
    pub project: DraftProject,
    pub datasets: Option<Vec<DraftDataset>>,
    pub persons: Option<Vec<DraftPerson>>,
    pub organizations: Option<Vec<DraftOrganization>>,
    pub grants: Option<Vec<DraftGrant>>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "__type")]
#[serde(rename = "Project")]
pub struct DraftProject {
    #[serde(rename = "__id")]
    pub id: String,
    #[serde(rename = "__createdAt")]
    pub created_at: Option<String>,
    #[serde(rename = "__createdBy")]
    pub created_by: Option<String>,
    pub shortcode: Shortcode,
    pub status: Option<DraftProjectStatus>,
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

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Hash, Eq)]
#[serde(try_from = "String")]
pub struct Shortcode(String);
impl Shortcode {
    pub fn as_string(&self) -> String {
        self.0.to_string()
    }
}
impl TryFrom<String> for Shortcode {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let regex: Regex = Regex::new(r"^[A-F0-9]{4}$").expect("Valid regex");
        let value = value.to_uppercase();
        if !regex.is_match(&value) {
            Err("Shortcode must be a 4 character hexadecimal string")
        } else {
            Ok(Shortcode(value))
        }
    }
}
#[test]
fn test_try_from_shortcode() {
    assert!(Shortcode::try_from("000F".to_string()).is_ok());
    assert!(Shortcode::try_from("12345".to_string()).is_err());
    assert!(Shortcode::try_from("000G".to_string()).is_err());
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub enum DraftProjectStatus {
    #[default]
    #[serde(rename = "ongoing")]
    Ongoing,
    #[serde(rename = "finished")]
    Finished,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DraftPublication {
    pub text: String,
    pub url: Option<NonEmpty<DraftUrl>>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "__type")]
#[serde(rename = "Dataset")]
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum DraftAccessCondition {
    Open,
    Restricted,
    Closed,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DraftStatus {
    #[serde(rename = "In planning")]
    InPlanning,
    #[serde(rename = "Ongoing")]
    OnGoing,
    #[serde(rename = "On hold")]
    OnHold,
    Finished,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DraftTypeOfData {
    XML,
    Text,
    Image,
    Video,
    Audio,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "__type")]
#[serde(rename = "Person")]
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

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "__type")]
#[serde(rename = "Organization")]
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

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "__type")]
#[serde(rename = "Grant")]
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DraftText(pub HashMap<DraftIsoCode, String>);

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Hash)]
#[serde(try_from = "String")]
pub struct DraftIsoCode(String);
impl DraftIsoCode {
    pub fn as_string(&self) -> String {
        self.0.to_string()
    }
}
impl TryFrom<String> for DraftIsoCode {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let regex: Regex = Regex::new(r"^[a-z]{2}$").expect("Valid regex");
        if !regex.is_match(&value) {
            Err("ISO code must be a 2 character lower case string")
        } else {
            Ok(DraftIsoCode(value))
        }
    }
}
#[test]
fn test_try_from_draft_iso_code() {
    assert!(DraftIsoCode::try_from("en".to_string()).is_ok());
    assert!(DraftIsoCode::try_from("de_ch".to_string()).is_err());
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DraftDate(pub NaiveDate);

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "__type")]
#[serde(rename = "URL")]
pub struct DraftUrl {
    pub url: String,
    pub text: Option<String>,
    #[serde(rename = "type")]
    #[serde(default = "DraftUrlType::default")]
    pub url_type: DraftUrlType,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "__type")]
#[serde(rename = "Address")]
pub struct DraftAddress {
    pub street: String,
    pub postal_code: String,
    pub locality: Option<String>,
    pub country: String,
    pub canton: Option<String>,
    pub additional: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "__type")]
#[serde(rename = "DataManagementPlan")]
pub struct DraftDataManagementPlan {
    pub available: bool,
    pub url: Option<DraftUrl>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "__type")]
#[serde(rename = "Attribution")]
pub struct DraftAttribution {
    pub agent: String,
    pub roles: NonEmpty<String>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "__type")]
#[serde(rename = "License")]
pub struct DraftLicense {
    pub license: DraftUrl,
    pub date: DraftDate,
    pub details: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum DraftTextOrUrl {
    TextValue(DraftText),
    UrlValue(DraftUrl),
}
