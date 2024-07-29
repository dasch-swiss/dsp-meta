use std::collections::HashMap;
use std::fs::File;
use std::path::Path;

use serde_json::Value;
use valico::json_schema::schema::ScopedSchema;
use valico::json_schema::{Scope, ValidationState};

use crate::domain::model::error::ValidationError::*;
use crate::domain::model::error::*;

static DRAFT_SCHEMA: &str = include_str!("../../../resources/schema-metadata-draft.json");
static FINAL_SCHEMA: &str = include_str!("../../../resources/schema-metadata-final.json");

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum SchemaVersion {
    Draft,
    Final,
}
impl SchemaVersion {
    fn schema_str(&self) -> &str {
        match self {
            SchemaVersion::Draft => DRAFT_SCHEMA,
            SchemaVersion::Final => FINAL_SCHEMA,
        }
    }
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
        let project_status = contents
            .get("project")
            .and_then(|p| p.get("status"))
            .and_then(|s| s.as_str());
        // Always validate against Draft schema
        // Only validate against Final schema if the project status is "finished"
        if schema_version == SchemaVersion::Draft
            || (schema_version == SchemaVersion::Final && (project_status == Some("finished")))
        {
            let state = schema.validate(&contents);
            results.insert(path, state);
        }
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
