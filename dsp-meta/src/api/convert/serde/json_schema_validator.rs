use std::collections::HashMap;
use std::fs::File;
use std::path::{Path};

use serde_json::Value;
use valico::json_schema::{Scope, ValidationState};
use valico::json_schema::schema::ScopedSchema;

use crate::api::convert::serde::json_schema_validator::SchemaVersion::Draft;
use crate::api::convert::serde::json_schema_validator::ValidationError::*;

static DRAFT_SCHEMA: &str = include_str!("../../../../resources/schema-metadata-draft.json");

pub enum SchemaVersion {
    Draft
}

pub type Result<T> = core::result::Result<T, ValidationError>;
#[derive(Debug)]
pub enum ValidationError {
    FileNotLoaded(std::io::Error),
    SchemaError(valico::json_schema::schema::SchemaError),
    NotAJsonFile(serde_json::Error),
}

pub fn validate_file(path: &Path, schema_version: SchemaVersion) -> Result<ValidationState> {
    let contents = load_path_as_json(path)?;
    let mut scope = Scope::new();
    let schema = load_json_schema(schema_version, &mut scope)?;
    Ok(schema.validate(&contents))
}

pub fn validate_files(paths: Vec<&Path>, schema_version: SchemaVersion) -> Result<HashMap<&Path, ValidationState>> {
    let mut scope = Scope::new();
    let schema = load_json_schema(schema_version, &mut scope)?;
    let mut results = HashMap::with_capacity(paths.len());
    for path in paths {
        let contents = load_path_as_json(path)?;
        let state = schema.validate(&contents);
        results.insert(path, state);
    }
    Ok(results)
}

fn load_path_as_json(path: &Path) -> Result<Value> {
    let file = File::open(path).map_err(|e| { FileNotLoaded(e) })?;
    let value = serde_json::from_reader::<File, Value>(file);
    value.map_err(|e| { NotAJsonFile(e) })
}

fn load_json_schema(schema_version: SchemaVersion, scope: &mut Scope) -> Result<ScopedSchema> {
    let schema_str = match schema_version { Draft => { DRAFT_SCHEMA } };
    let json = serde_json::from_str(schema_str).map_err(|e| NotAJsonFile(e))?;
    scope.compile_and_return(json, false).map_err(|e| { SchemaError(e) })
}