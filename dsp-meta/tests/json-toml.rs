use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::path::PathBuf;

use nonempty::NonEmpty;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use valico::json_schema;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
    pub project: Project,
    pub datasets: Option<Vec<Dataset>>,
    pub persons: Option<Vec<Person>>,
    pub organizations: Option<Vec<Organization>>,
    pub grants: Option<Vec<Grant>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    #[serde(rename = "__id")]
    pub id: String,
    #[serde(rename = "__createdAt")]
    pub created_at: Option<String>,
    #[serde(rename = "__createdBy")]
    pub created_by: Option<String>,
    pub shortcode: String,
    pub name: String,
    pub description: Option<Text>,
    pub start_date: Date,
    pub teaser_text: String,
    pub datasets: NonEmpty<String>,
    pub keywords: NonEmpty<Text>,
    pub disciplines: NonEmpty<TextOrUrl>,
    pub temporal_coverage: Option<NonEmpty<TextOrUrl>>,
    pub spatial_coverage: Option<NonEmpty<Url>>,
    pub funders: Option<NonEmpty<String>>,
    pub url: Option<Url>,
    pub secondary_url: Option<Url>,
    pub data_management_plan: Option<DataManagementPlan>,
    pub end_date: Option<Date>,
    pub contact_point: Option<String>,
    pub how_to_cite: Option<String>,
    pub publications: Option<NonEmpty<Publication>>,
    pub grants: Option<NonEmpty<String>>,
    pub alternative_names: Option<NonEmpty<Text>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Publication {
    pub text: String,
    pub url: Option<NonEmpty<Url>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Dataset {
    #[serde(rename = "__id")]
    pub id: String,
    #[serde(rename = "__createdAt")]
    pub created_at: Option<String>,
    #[serde(rename = "__createdBy")]
    pub created_by: Option<String>,
    pub abstracts: Option<NonEmpty<TextOrUrl>>,
    pub access_conditions: Option<AccessCondition>,
    pub additional: Option<Vec<TextOrUrl>>,
    pub alternative_titles: Option<Vec<Text>>,
    pub attributions: Option<NonEmpty<Attribution>>,
    pub date_created: Option<Date>,
    pub date_modified: Option<Date>,
    pub date_published: Option<Date>,
    pub distribution: Option<Url>,
    pub how_to_cite: Option<String>,
    pub languages: Option<NonEmpty<Text>>,
    pub licenses: Option<NonEmpty<License>>,
    pub status: Option<Status>,
    pub title: Option<String>,
    pub type_of_data: Option<NonEmpty<TypeOfData>>,
    pub urls: Option<Vec<Url>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum AccessCondition {
    Open,
    Restricted,
    Closed,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum Status {
    #[serde(rename = "In planning")]
    InPlanning,
    #[serde(rename = "Ongoing")]
    OnGoing,
    #[serde(rename = "On hold")]
    OnHold,
    Finished,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum TypeOfData {
    XML,
    Text,
    Image,
    Video,
    Audio,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Person {
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
    pub address: Option<Address>,
    pub email: Option<String>,
    pub secondary_email: Option<String>,
    pub authority_refs: Option<Vec<Url>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Organization {
    #[serde(rename = "__id")]
    pub id: String,
    #[serde(rename = "__createdAt")]
    pub created_at: Option<String>,
    #[serde(rename = "__createdBy")]
    pub created_by: Option<String>,
    pub name: String,
    pub url: Option<Url>,
    pub address: Option<Address>,
    pub email: Option<String>,
    pub alternative_names: Option<Vec<Text>>,
    pub authority_refs: Option<Vec<Url>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Grant {
    #[serde(rename = "__id")]
    pub id: String,
    #[serde(rename = "__createdAt")]
    pub created_at: Option<String>,
    #[serde(rename = "__createdBy")]
    pub created_by: Option<String>,
    pub funders: NonEmpty<String>,
    pub number: Option<String>,
    pub name: Option<String>,
    pub url: Option<Url>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Text(HashMap<IsoCode, String>);

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub struct IsoCode(String);

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Date(String);

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Url {
    pub url: String,
    pub text: Option<String>,
    #[serde(rename = "type")]
    #[serde(default = "UrlType::default")]
    pub url_type: UrlType,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum UrlType {
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
impl UrlType {
    fn default() -> Self { UrlType::URL }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Address {
    pub street: String,
    pub postal_code: String,
    pub locality: Option<String>,
    pub country: String,
    pub canton: Option<String>,
    pub additional: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DataManagementPlan {
    pub available: bool,
    pub url: Option<Url>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Attribution {
    pub agent: String,
    pub roles: NonEmpty<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct License {
    pub license: Url,
    pub date: Date,
    pub details: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum TextOrUrl {
    TextValue(Text),
    UrlValue(Url),
}

#[test]
fn test_as_toml_and_yaml() {
    let path = "/Users/christian/git/dasch/dsp-meta/data/examples/sgv.json";
    let contents = fs::read_to_string(path)
        .expect("Should have been able to read the file");
    let metadata = serde_json::from_str::<Metadata>(&*contents).expect("From JSON");
    let toml = toml::to_string(&metadata).expect("To TOML");
    fs::write("/Users/christian/git/dasch/dsp-meta/data/examples/sgv.toml", &toml).expect("Write TOML");
    println!("As TOML:");
    println!("{}", toml);
    println!();

    let yaml = serde_yaml::to_string(&metadata).expect("To YAML");
    fs::write("/Users/christian/git/dasch/dsp-meta/data/examples/sgv.yaml", &yaml).expect("Write YAML");
    println!("As YAML:");
    println!("{}", yaml);
    println!("As YAML:");
}

#[test]
fn test_json_and_yaml_serialization_are_equal() {
    let path = "/Users/christian/git/dasch/dsp-meta/data/examples/sgv.json";
    let contents_json = fs::read_to_string(path).expect("Read JSON");
    let metadata_json = serde_json::from_str::<Metadata>(&*contents_json).expect("From JSON");
    let contents_yaml = fs::read_to_string("/Users/christian/git/dasch/dsp-meta/data/examples/sgv.yaml").expect("Read YML");
    let metadata_yaml = serde_yaml::from_str(&*contents_yaml).expect("From YAML");
    assert_eq!(metadata_json, metadata_yaml);
}

#[test]
fn test_json_and_toml_serialization_are_equal() {
    let path = "/Users/christian/git/dasch/dsp-meta/data/examples/sgv.json";
    let contents_json = fs::read_to_string(path).expect("Read JSON");
    let metadata_json = serde_json::from_str::<Metadata>(&*contents_json).expect("From JSON");
    let contents_toml = fs::read_to_string("/Users/christian/git/dasch/dsp-meta/data/examples/sgv.toml").expect("Read TOML");
    let metadata_toml = toml::from_str::<Metadata>(&*contents_toml).expect("From TOML");
    assert_eq!(metadata_json, metadata_toml);
}

#[test]
fn test_deserialization() {
    let paths = fs::read_dir("/Users/christian/git/dasch/dsp-meta/data/json")
        .expect("Directory not found")
        .filter_map(
            |entry| {
                let entry = entry.ok()?;
                let path = entry.path();
                if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("json") {
                    Some(path)
                } else {
                    None
                }
            }
        ).collect::<Vec<PathBuf>>();
    let mut success: usize = 0;
    let mut error: usize = 0;

    for path in paths {
        let path = path.as_path();
        if path.extension().and_then(|s| s.to_str()) == Some("json") {
            println!("Checking {}:", path.to_str().get_or_insert(""));
            let contents = fs::read_to_string(path)
                .expect("Should have been able to read the file");
            let metadata = serde_json::from_str::<Metadata>(&*contents);
            match metadata {
                Ok(_data) => {
                    success = success + 1;
                    println!("SUCCESS\n") // println!("DATA:\n {:?}\n", data),
                }
                Err(err) => {
                    error = error + 1;
                    println!("ERROR:\n {:?}\n", err)
                }
            };
        }
    }
    println!("Success: {}, Error: {}, Total: {}", success, error, success + error)
}

#[test]
fn test_jsonschema() {
    verify_all_json_files_in_directory_jsonschema("/Users/christian/git/dasch/dsp-meta/data/json/");
    assert!(true)
}

fn verify_all_json_files_in_directory_jsonschema(directory: &str) {
    let paths = fs::read_dir(directory).unwrap();
    let mut success: usize = 0;
    let mut error: usize = 0;
    let json_schema: Value = serde_json::from_reader(File::open("/Users/christian/git/dasch/dsp-meta/data/schema-metadata-draft.json").unwrap()).unwrap();
    let mut scope = json_schema::Scope::new();
    let schema = scope.compile_and_return(json_schema, false).unwrap();
    let mut valid: Vec<String> = Vec::new();
    let mut invalid: Vec<String> = Vec::new();

    for path in paths {
        let path = path.unwrap().path();
        if path.extension().and_then(|s| s.to_str()) == Some("json") {
            let file = (*path.to_str().get_or_insert("")).to_string();
            println!("Checking {}:", file);
            let contents = fs::read_to_string(&path)
                .expect("Should have been able to read the file");
            let metadata = serde_json::from_str::<Value>(&*contents).expect("parsed data as json");
            let result = schema.validate(&metadata);
            let filename = file["/Users/christian/git/dasch/dsp-meta/data/json/".len()..].to_string();
            if result.is_valid() {
                success = success + 1;
                valid.push(filename);
                println!("VALID\n") // println!("DATA:\n {:?}\n", data),
            } else {
                error = error + 1;
                invalid.push(filename);
                println!("INVALID: {:?}\n", result) // println!("DATA:\n {:?}\n", data),
            }
        }
    }
    println!("Success: {}, Error: {}, Total: {}", success, error, success + error);
    println!();

    println!("VALID files:\n{}", valid.join("\n"));
    println!();

    println!("INVALID files:\n{}", invalid.join("\n"));
}