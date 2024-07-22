use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::path::Path;
use chrono::{DateTime, Utc};
use nonempty::NonEmpty;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use valico::json_schema;
use crate::TextOrUrl::{TextValue, UrlValue};

mod timestamp_nanos_date_format {
    use chrono::{DateTime, Utc};
    use serde::{de, Deserialize, Deserializer, ser, Serializer};

    pub fn serialize<S>(
        date: &DateTime<Utc>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let ts_nanos = date.timestamp_nanos_opt()
            .ok_or(ser::Error::custom(format!("date out of range: {}", date)))?;
        serializer.serialize_str(&format!("{}", ts_nanos))
    }
    pub fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let ts = String::deserialize(deserializer)?;
        let ts_millis: i64 = ts.parse::<i64>()
            .map_err(|e| de::Error::custom(format!("invalid timestamp: {} {}", ts, e)))?;
        Ok(DateTime::from_timestamp_nanos(ts_millis))
    }
}
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
    pub project: Project,
    pub datasets: NonEmpty<Dataset>,
    pub persons: Option<Vec<Person>>,
    pub organizations: Option<Vec<Organization>>,
    pub grants: Option<Vec<Grant>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    #[serde(rename(serialize = "id", deserialize = "__id"))]
    pub id: String,
    #[serde(with = "timestamp_nanos_date_format")]
    #[serde(rename(serialize = "createdAt", deserialize = "__createdAt"))]
    pub created_at: DateTime<Utc>,
    #[serde(rename(serialize = "createdBy", deserialize = "__createdBy"))]
    pub created_by: String,
    pub shortcode: String,
    pub name: String,
    pub description: Text,
    pub start_date: Date,
    pub teaser_text: String,
    pub datasets: NonEmpty<String>,
    pub keywords: NonEmpty<Text>,
    pub disciplines: NonEmpty<TextOrUrl>,
    pub temporal_coverage: NonEmpty<TextOrUrl>,
    pub spatial_coverage: NonEmpty<Url>,
    pub funders: NonEmpty<String>,
    pub url: Url,
    pub secondary_url: Option<Url>,
    pub data_management_plan: Option<DataManagementPlan>,
    pub end_date: Option<Date>,
    pub contact_point: Option<String>,
    pub how_to_cite: String,
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
    #[serde(rename(serialize = "id", deserialize = "__id"))]
    pub id: String,
    #[serde(with = "timestamp_nanos_date_format")]
    #[serde(rename(serialize = "createdAt", deserialize = "__createdAt"))]
    pub created_at: DateTime<Utc>,
    #[serde(rename(serialize = "createdBy", deserialize = "__createdBy"))]
    pub created_by: String,
    pub title: String,
    pub access_conditions: AccessCondition,
    pub how_to_cite: String,
    pub status: Status,
    pub abstracts: NonEmpty<TextOrUrl>,
    pub type_of_data: NonEmpty<TypeOfData>,
    pub licenses: NonEmpty<License>,
    pub languages: NonEmpty<Text>,
    pub attributions: NonEmpty<Attribution>,
    pub alternative_titles: Option<Vec<Text>>,
    pub date_published: Option<Date>,
    pub date_created: Option<Date>,
    pub date_modified: Option<Date>,
    pub distribution: Option<Url>,
    pub urls: Option<Vec<Url>>,
    pub additional: Option<Vec<TextOrUrl>>,
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
    #[serde(rename(serialize = "id", deserialize = "__id"))]
    pub id: String,
    #[serde(with = "timestamp_nanos_date_format")]
    #[serde(rename(serialize = "createdAt", deserialize = "__createdAt"))]
    pub created_at: DateTime<Utc>,
    #[serde(rename(serialize = "createdBy", deserialize = "__createdBy"))]
    pub created_by: String,
    pub job_titles: NonEmpty<String>,
    pub given_names: NonEmpty<String>,
    pub family_names: NonEmpty<String>,
    pub affiliation: NonEmpty<String>,
    pub address: Option<Address>,
    pub email: Option<String>,
    pub secondary_email: Option<String>,
    pub authority_refs: Option<Vec<Url>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Organization {
    #[serde(rename(serialize = "id", deserialize = "__id"))]
    pub id: String,
    #[serde(with = "timestamp_nanos_date_format")]
    #[serde(rename(serialize = "createdAt", deserialize = "__createdAt"))]
    pub created_at: DateTime<Utc>,
    #[serde(rename(serialize = "createdBy", deserialize = "__createdBy"))]
    pub created_by: String,
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
    #[serde(rename(serialize = "id", deserialize = "__id"))]
    pub id: String,
    #[serde(with = "timestamp_nanos_date_format")]
    #[serde(rename(serialize = "createdAt", deserialize = "__createdAt"))]
    pub created_at: DateTime<Utc>,
    #[serde(rename(serialize = "createdBy", deserialize = "__createdBy"))]
    pub created_by: String,
    pub funders: NonEmpty<String>,
    pub number: Option<String>,
    pub name: Option<String>,
    pub url: Option<Url>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Text(HashMap<String, String>);

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
    pub locality: String,
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

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Untagged {
    the_enum: NonEmpty<TextOrUrl>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct P {
    some_str: Option<String>,
    p: Untagged,
}

#[test]
fn untagged_enum() {
    let mut val =
        NonEmpty::new(
            UrlValue(Url {
                url: "http://example.com".to_string(),
                text: Some("text".to_string()),
                url_type: UrlType::URL,
            }));
    val.push(TextValue(Text([("en".to_string(), "English".to_string())].iter().cloned().collect())));
    let un = P {
        some_str: None,
        p: Untagged {
            the_enum: val
        },
    };
    let foo = toml::to_string(&un).expect("To TOML");
    println!("{}", foo);
}

#[test]
fn test_as_toml() {
    let path = "/Users/christian/git/dasch/dsp-meta/data/json/beol.json";
    let contents = fs::read_to_string(path)
        .expect("Should have been able to read the file");
    let metadata = serde_json::from_str::<Metadata>(&*contents).expect("From JSON");
    let foo = toml::to_string(&metadata).expect("To TOML");
    println!("{}", foo);
    println!();
    let foo = serde_json::to_string(&metadata).expect("To JSON");
    println!("{}", foo);
}

#[test]
fn test_deserialization() {
    let paths = vec![
        "/Users/christian/git/dasch/dsp-meta/data/json/_bilddatenbank.json",
        "/Users/christian/git/dasch/dsp-meta/data/json/fagottino.json",
        "/Users/christian/git/dasch/dsp-meta/data/json/roud.json",
        "/Users/christian/git/dasch/dsp-meta/data/json/limc.json",
        "/Users/christian/git/dasch/dsp-meta/data/json/posepi.json",
        "/Users/christian/git/dasch/dsp-meta/data/json/olympic.json",
        "/Users/christian/git/dasch/dsp-meta/data/json/dasch.json",
        "/Users/christian/git/dasch/dsp-meta/data/json/igeoarchive.json",
        "/Users/christian/git/dasch/dsp-meta/data/json/mfmps.json",
        "/Users/christian/git/dasch/dsp-meta/data/json/digitalagenda.json",
        "/Users/christian/git/dasch/dsp-meta/data/json/waldaucinema.json",
        "/Users/christian/git/dasch/dsp-meta/data/json/operativetv.json",
        "/Users/christian/git/dasch/dsp-meta/data/json/beol.json",
        "/Users/christian/git/dasch/dsp-meta/data/json/lenzburg.json",
        "/Users/christian/git/dasch/dsp-meta/data/json/_dssl.json",
        "/Users/christian/git/dasch/dsp-meta/data/json/reforme-geneve.json",
        "/Users/christian/git/dasch/dsp-meta/data/json/awg.json",
        "/Users/christian/git/dasch/dsp-meta/data/json/religious-speech.json",
        "/Users/christian/git/dasch/dsp-meta/data/json/cache.json",
        "/Users/christian/git/dasch/dsp-meta/data/json/stardom.json",
        "/Users/christian/git/dasch/dsp-meta/data/json/prom_know.json",
        "/Users/christian/git/dasch/dsp-meta/data/json/mls.json",
        "/Users/christian/git/dasch/dsp-meta/data/json/aura-effizienz.json",
        "/Users/christian/git/dasch/dsp-meta/data/json/drawings.json",
        "/Users/christian/git/dasch/dsp-meta/data/json/tdk.json",
        "/Users/christian/git/dasch/dsp-meta/data/json/societesavoie.json",
        "/Users/christian/git/dasch/dsp-meta/data/json/biz.json",
        "/Users/christian/git/dasch/dsp-meta/data/json/big-data-in-agriculture.json",
        "/Users/christian/git/dasch/dsp-meta/data/json/tds.json",
        "/Users/christian/git/dasch/dsp-meta/data/json/_rosetta.json",
        "/Users/christian/git/dasch/dsp-meta/data/json/mark16.json",
        "/Users/christian/git/dasch/dsp-meta/data/json/hdm.json",
        "/Users/christian/git/dasch/dsp-meta/data/json/globalgeschichte.json",
    ].into_iter() .map(|s| Path::new(s));
    let mut success: usize = 0;
    let mut error: usize = 0;

    for path in paths {
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
    let json_schema: Value = serde_json::from_reader(File::open("/Users/christian/git/dasch/dsp-meta/docs/domain_model/schema-metadata.json").unwrap()).unwrap();
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