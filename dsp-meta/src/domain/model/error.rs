use thiserror::Error;

pub type Result<T> = core::result::Result<T, ValidationError>;

#[derive(Debug, Error)]
pub enum ValidationError {
    #[error("File not loaded: {0}")]
    FileNotLoaded(std::io::Error),
    #[error("Schema is invalid: {0}")]
    SchemaError(valico::json_schema::schema::SchemaError),
    #[error("Error parsing file as json: {0}")]
    NotAJsonFile(serde_json::Error),
}
