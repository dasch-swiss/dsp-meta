use std::path::{Path, PathBuf};
use std::{env, fs};

use api::convert::serde::draft_model::*;
use dsp_meta::api;
use dsp_meta::api::convert::serde::json_schema_validator::{validate_files, SchemaVersion};

#[test]
fn test_json_and_yaml_serialization_are_equal() {
    let mut path = data_dir();
    path.push("json");
    path.push("sgv.json");
    let contents_json = fs::read_to_string(path).expect("Read JSON");
    let metadata_json = serde_json::from_str::<DraftMetadata>(&*contents_json).expect("From JSON");
    let contents_yaml = serde_yaml::to_string(&metadata_json).expect("To YAML");
    let metadata_yaml = serde_yaml::from_str(&*contents_yaml).expect("From YAML");
    assert_eq!(metadata_json, metadata_yaml);
}

#[test]
fn test_json_and_toml_serialization_are_equal() {
    let mut path = data_dir();
    path.push("json");
    path.push("sgv.json");
    let contents_json = fs::read_to_string(path).expect("Read JSON");
    let metadata_json = serde_json::from_str::<DraftMetadata>(&*contents_json).expect("From JSON");
    let contents_toml = toml::to_string(&metadata_json).expect("To TOML");
    let metadata_toml = toml::from_str::<DraftMetadata>(&*contents_toml).expect("From TOML");
    assert_eq!(metadata_json, metadata_toml);
}

#[test]
fn test_deserialization_all_json_data() {
    let paths = collect_data_json_paths();
    let mut success: usize = 0;
    let mut error: usize = 0;

    for path in paths {
        let path = path.as_path();
        println!("Checking {}:", path.to_str().get_or_insert(""));
        let contents = fs::read_to_string(path).expect("Should have been able to read the file");
        let metadata = serde_json::from_str::<DraftMetadata>(&*contents);
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
    println!(
        "Success: {}, Error: {}, Total: {}",
        success,
        error,
        success + error
    )
}

fn collect_data_json_paths() -> Vec<PathBuf> {
    let mut current_dir = data_dir();
    current_dir.push("json");
    fs::read_dir(current_dir)
        .expect("Failed to read data_dir")
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("json") {
                Some(path)
            } else {
                None
            }
        })
        .collect::<Vec<PathBuf>>()
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
    let path_bufs = collect_data_json_paths();
    let paths: Vec<&Path> = path_bufs.iter().map(|p| p.as_path()).collect();
    let results = validate_files(paths, SchemaVersion::Draft).unwrap();
    for (key, value) in &results {
        println!("{:?}: {:?}", key, value);
    }
    let failure_count = results.values().filter(|v| !v.is_valid()).count();
    assert_eq!(failure_count, 0);
}
