use std::collections::HashMap;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::{env, fs};

use api::convert::serde::draft_model::*;
use dsp_meta::api;
use dsp_meta::api::convert::serde::json_schema_validator::{validate_files, SchemaVersion};
use dsp_meta::infrastructure::load_json_file_paths;

#[test]
fn test_json_and_yaml_serialization_are_equal() {
    let mut sgv_json = data_dir();
    sgv_json.push("json");
    sgv_json.push("sgv.json");
    let contents_json = fs::read_to_string(sgv_json).expect("Read JSON");
    let metadata_json = serde_json::from_str::<DraftMetadata>(&*contents_json).expect("From JSON");
    let contents_yaml = serde_yaml::to_string(&metadata_json).expect("To YAML");
    let metadata_yaml = serde_yaml::from_str(&*contents_yaml).expect("From YAML");
    assert_eq!(metadata_json, metadata_yaml);
}

#[test]
fn test_json_and_toml_serialization_are_equal() {
    let mut sgv_json = data_dir();
    sgv_json.push("json");
    sgv_json.push("sgv.json");
    let contents_json = fs::read_to_string(sgv_json).expect("Read JSON");
    let metadata_json = serde_json::from_str::<DraftMetadata>(&*contents_json).expect("From JSON");
    let contents_toml = toml::to_string(&metadata_json).expect("To TOML");
    let metadata_toml = toml::from_str::<DraftMetadata>(&*contents_toml).expect("From TOML");
    assert_eq!(metadata_json, metadata_toml);
}

#[test]
fn test_deserialization_all_json_data() {
    let json_file_paths = load_json_file_paths(&data_dir());
    let nr_of_files = json_file_paths.len();
    let mut success: usize = 0;
    let mut error: usize = 0;

    let mut deserialized: Vec<DraftMetadata> = Vec::new();
    for path in json_file_paths {
        let path = path.as_path();
        let contents = fs::read_to_string(path).expect("Should have been able to read the file");
        let metadata = serde_json::from_str::<DraftMetadata>(&*contents);
        match metadata {
            Ok(_data) => {
                deserialized.push(_data);
                success = success + 1;
            }
            Err(err) => {
                error = error + 1;
                println!("Checked {}:", path.to_str().get_or_insert(""));
                println!("ERROR:\n {:?}\n", err)
            }
        };
    }
    println!(
        "Success: {}, Error: {}, Total: {}",
        success,
        error,
        success + error
    );
    assert_eq!(deserialized.len(), nr_of_files)
}

fn data_dir() -> PathBuf {
    let mut current_dir = env::current_dir()
        .ok()
        .and_then(|e| e.parent().map(|p| p.to_path_buf()))
        .expect("Project root dir");
    current_dir.push("data");
    current_dir
}

#[test]
fn test_draft_json_schema() {
    let path_bufs = load_json_file_paths(&data_dir());
    let paths: Vec<&Path> = path_bufs.iter().map(|p| p.as_path()).collect();
    let results = validate_files(paths, SchemaVersion::Draft).unwrap();
    for (key, value) in &results {
        if !value.is_valid() {
            dbg!("{:?}: {:?}", key, value);
        }
    }
    let failures = results.values().filter(|v| !v.is_valid());
    assert_eq!(failures.count(), 0);
}

#[test]
fn test_unique_shortcode() {
    let path_bufs = load_json_file_paths(&data_dir());
    let mut shortcodes: HashMap<String, PathBuf> = HashMap::new();
    for path_buf in path_bufs {
        let path = path_buf.as_path();
        let file = File::open(path).expect("Should have been able to open the file");
        let metadata: DraftMetadata =
            serde_json::from_reader(file).expect("Should have been able to deserialize the file");
        let shortcode = metadata.project.shortcode;
        if shortcodes.contains_key(&shortcode) {
            let last_path = shortcodes.get(&shortcode).unwrap();
            panic!(
                "Shortcode {} is not unique!\nFound in files '{}' and '{}'. Please fix this.",
                shortcode,
                path.file_name().and_then(|os| os.to_str()).unwrap_or_default(),
                last_path.file_name().and_then(|os| os.to_str()).unwrap_or_default()
            );
        }
        shortcodes.insert(shortcode, path_buf);
    }
}
