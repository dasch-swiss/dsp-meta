use std::fs;
use std::fs::File;
use std::path::PathBuf;

use serde_json::Value;
use valico::json_schema;
use api::convert::serde::draft_model::*;
use dsp_meta::api;

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
fn test_deserialization_data() {
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
fn test_draft_json_schema() {
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

    assert!(invalid.is_empty());
}