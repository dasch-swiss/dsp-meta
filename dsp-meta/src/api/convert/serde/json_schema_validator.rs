use std::collections::HashMap;
use std::fs::File;
use std::path::Path;

use serde_json::Value;
use thiserror::Error;
use valico::json_schema::schema::ScopedSchema;
use valico::json_schema::{Scope, ValidationState};

use crate::api::convert::serde::json_schema_validator::SchemaVersion::Draft;
use crate::api::convert::serde::json_schema_validator::ValidationError::*;

static DRAFT_SCHEMA: &str = include_str!("../../../../resources/schema-metadata-draft.json");

pub enum SchemaVersion {
    Draft,
}
impl SchemaVersion {
    fn schema_str(&self) -> &str {
        match self {
            Draft => DRAFT_SCHEMA,
        }
    }
}

pub type Result<T> = core::result::Result<T, ValidationError>;
#[derive(Debug, Error )]
pub enum ValidationError {
    #[error("File not loaded: {0}")]
    FileNotLoaded(std::io::Error),
    #[error("Schema is invalid: {0}")]
    SchemaError(valico::json_schema::schema::SchemaError),
    #[error("Error parsing file as json: {0}")]
    NotAJsonFile(serde_json::Error),
}

pub fn validate_file(path: &Path, schema_version: SchemaVersion) -> Result<ValidationState> {
    let contents = load_path_as_json(path)?;
    let mut scope = Scope::new();
    let schema = load_json_schema(schema_version, &mut scope)?;
    Ok(schema.validate(&contents))
}

pub fn validate_files(
    paths: Vec<&Path>,
    schema_version: SchemaVersion,
) -> Result<HashMap<&Path, ValidationState>> {
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
    let file = File::open(path).map_err(FileNotLoaded)?;
    let value = serde_json::from_reader::<File, Value>(file);
    value.map_err(NotAJsonFile)
}

fn load_json_schema(schema_version: SchemaVersion, scope: &mut Scope) -> Result<ScopedSchema> {
    let schema_str = schema_version.schema_str();
    let json = serde_json::from_str(schema_str).map_err(NotAJsonFile)?;
    scope.compile_and_return(json, false).map_err(SchemaError)
}
