use std::io;

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum DspMetaError {
    IO(String),
    ParseHcl(String),
    UnknownAttribute(String),
    ParseVersion(String),
    ParseProject(String),
    ParseDataset(String),
    CreateValueObject(String),
    NotFound,
}

impl From<io::Error> for DspMetaError {
    fn from(error: io::Error) -> Self {
        DspMetaError::IO(error.to_string())
    }
}

impl From<hcl::Error> for DspMetaError {
    fn from(error: hcl::Error) -> Self {
        DspMetaError::ParseHcl(error.to_string())
    }
}

/// Convert `DspMetaError` into a response.
/// TODO: Add correct status codes and error messages.
impl IntoResponse for DspMetaError {
    fn into_response(self) -> Response {
        match self {
            DspMetaError::IO(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Something went wrong: {}", err),
            )
                .into_response(),
            DspMetaError::ParseHcl(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Something went wrong: {}", err),
            )
                .into_response(),
            DspMetaError::UnknownAttribute(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Something went wrong: {}", err),
            )
                .into_response(),
            DspMetaError::ParseVersion(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Something went wrong: {}", err),
            )
                .into_response(),
            DspMetaError::ParseProject(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Something went wrong: {}", err),
            )
                .into_response(),
            DspMetaError::ParseDataset(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Something went wrong: {}", err),
            )
                .into_response(),
            DspMetaError::CreateValueObject(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Something went wrong: {}", err),
            )
                .into_response(),
            DspMetaError::NotFound => (StatusCode::NOT_FOUND, "Not Found").into_response(),
        }
    }
}
