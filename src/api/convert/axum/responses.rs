use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

use crate::error::DspMetaError;

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
            DspMetaError::ParseGrant(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Something went wrong: {}", err),
            )
                .into_response(),
            DspMetaError::CreateDomainObject(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong:").into_response()
            }
            DspMetaError::CreateValueObject(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Something went wrong: {}", err),
            )
                .into_response(),
            DspMetaError::SerializeToRdf(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response()
            }
            DspMetaError::NotFound => (StatusCode::NOT_FOUND, "Not Found").into_response(),
            DspMetaError::JsonSerialization(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Error serializing response to JSON: {}", err),
            )
                .into_response(),
        }
    }
}
