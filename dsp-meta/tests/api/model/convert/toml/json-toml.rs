use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Metadata {
    pub project: Project,
    pub datasets: Vec<Dataset>,
    pub persons: Option<Vec<Person>>,
    pub organizations: Option<Vec<Organization>>,
    pub grants: Option<Vec<Grant>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    #[serde(rename = "__id")]
    pub id: String,
    #[serde(rename = "__type")]
    pub p_type: String,
    #[serde(rename = "__createdAt")]
    pub created_at: String,
    #[serde(rename = "__createdBy")]
    pub created_by: String,
    #[serde(rename = "__modifiedAt")]
    pub modified_at: Option<String>,
    #[serde(rename = "__modifiedBy")]
    pub modified_by: Option<String>,
    #[serde(rename = "__deletedAt")]
    pub deleted_at: Option<String>,
    #[serde(rename = "__deletedBy")]
    pub deleted_by: Option<String>,
    pub shortcode: String,
    pub name: String,
    pub description: Text,
    pub start_date: Date,
    pub teaser_text: String,
    pub datasets: Vec<String>, // non empty
    pub keywords: Vec<Text>, // non empty
    pub disciplines: Vec<TextOrUrl>, // non empty
    pub temporal_coverage: Vec<TextOrUrl>, // non empty
    pub spatial_coverage: Vec<Url>,// non empty
    pub funders: Vec<String>,// non empty
    pub url: Url,
    pub secondary_url: Option<Url>,
    pub data_management_plan: Option<DataManagementPlan>,
    pub end_date: Option<Date>,
    pub contact_point: Option<String>,
    pub how_to_cite: String,
    pub publications: Option<Vec<String>>,
    pub grants: Option<Vec<String>>,
    pub alternative_names: Option<Vec<Text>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Dataset {
    #[serde(rename = "__id")]
    pub id: String,
    #[serde(rename = "__type")]
    pub d_type: String,
    #[serde(rename = "__createdAt")]
    pub created_at: String,
    #[serde(rename = "__createdBy")]
    pub created_by: String,
    #[serde(rename = "__modifiedAt")]
    pub modified_at: String,
    #[serde(rename = "__modifiedBy")]
    pub modified_by: String,
    #[serde(rename = "__deletedAt")]
    pub deleted_at: String,
    #[serde(rename = "__deletedBy")]
    pub deleted_by: String,
    pub title: String,
    pub access_conditions: String, // "enum": [ "open", "restricted", "closed" ]
    pub how_to_cite: String,
    pub status: String, // "enum": [ "In planning", "Ongoing", "On hold", "Finished" ]
    pub abstracts: Vec<TextOrUrl>,
    pub type_of_data: Vec<String>, // "enum": [ "XML", "Text", "Image", "Video", "Audio" ]
    pub licenses: Vec<License>,
    pub languages: Vec<Text>,
    pub attributions: Vec<Attribution>, // non-empty
    pub alternative_titles: Option<Vec<Text>>,
    pub date_published: Option<Date>,
    pub date_created: Option<Date>,
    pub date_modified: Option<Date>,
    pub distribution: Option<Url>,
    pub urls: Option<Vec<Url>>,
    pub additional: Option<Vec<TextOrUrl>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Person {
    #[serde(rename = "__id")]
    pub id: String,
    #[serde(rename = "__type")]
    pub p_type: String,
    #[serde(rename = "__createdAt")]
    pub created_at: String,
    #[serde(rename = "__createdBy")]
    pub created_by: String,
    #[serde(rename = "__modifiedAt")]
    pub modified_at: String,
    #[serde(rename = "__modifiedBy")]
    pub modified_by: String,
    #[serde(rename = "__deletedAt")]
    pub deleted_at: String,
    #[serde(rename = "__deletedBy")]
    pub deleted_by: String,
    pub job_titles: Vec<String>, // non empty
    pub given_names: Vec<String>, // non empty
    pub family_names: Vec<String>,// non empty
    pub affiliation: Vec<String>, // non empty
    pub address: Option<Address>,
    pub email: Option<String>,
    pub secondary_email: Option<String>,
    pub authority_refs: Option<Vec<Url>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Organization {
    #[serde(rename = "__id")]
    pub id: String,
    #[serde(rename = "__type")]
    pub o_type: String,
    #[serde(rename = "__createdAt")]
    pub created_at: String,
    #[serde(rename = "__createdBy")]
    pub created_by: String,
    #[serde(rename = "__modifiedAt")]
    pub modified_at: String,
    #[serde(rename = "__modifiedBy")]
    pub modified_by: String,
    #[serde(rename = "__deletedAt")]
    pub deleted_at: String,
    #[serde(rename = "__deletedBy")]
    pub deleted_by: String,
    pub name: String,
    pub url: Option<Url>,
    pub address: Option<Address>,
    pub email: Option<String>,
    pub alternative_names: Option<Vec<Text>>,
    pub authority_refs: Option<Vec<Url>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Grant {
    #[serde(rename = "__id")]
    pub id: String,
    #[serde(rename = "__type")]
    pub g_type: String,
    #[serde(rename = "__createdAt")]
    pub created_at: String,
    #[serde(rename = "__createdBy")]
    pub created_by: String,
    #[serde(rename = "__modifiedAt")]
    pub modified_at: String,
    #[serde(rename = "__modifiedBy")]
    pub modified_by: String,
    #[serde(rename = "__deletedAt")]
    pub deleted_at: String,
    #[serde(rename = "__deletedBy")]
    pub deleted_by: String,
    pub funders: Vec<String>, // non empty
    pub number: Option<String>,
    pub name: Option<String>,
    pub url: Option<Url>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Text(HashMap<String, String>);

#[derive(Debug, Serialize, Deserialize)]
pub struct Date(String);

#[derive(Debug, Serialize, Deserialize)]
pub struct Url {
    #[serde(rename = "__type")]
    pub u_type: String,
    pub r#type: String,
    pub url: String,
    pub text: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Address {
    #[serde(rename = "__type")]
    pub a_type: String,
    pub street: String,
    pub postal_code: String,
    pub locality: String,
    pub country: String,
    pub canton: Option<String>,
    pub additional: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DataManagementPlan {
    #[serde(rename = "__type")]
    pub dmp_type: String,
    pub available: bool,
    pub url: Option<Url>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Attribution {
    #[serde(rename = "__type")]
    pub a_type: String,
    pub agent: String,
    pub roles: Vec<String>, // non-empty
}

#[derive(Debug, Serialize, Deserialize)]
pub struct License {
    #[serde(rename = "__type")]
    pub l_type: String,
    pub license: Url,
    pub date: Date,
    pub details: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TextOrUrl {
    Text(Text),
    Url(Url),
}