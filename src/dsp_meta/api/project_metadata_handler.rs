use std::sync::Arc;

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::Json;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tracing::trace;

use crate::app_state::AppState;
use crate::domain::model::entity::project_metadata::ProjectMetadata;
use crate::domain::model::value::Shortcode;
use crate::domain::service::project_metadata_api_contract::ProjectMetadataApiContract;
use crate::errors::DspMetaError;

/// GET /project_metadata/:shortcode
/// Get project metadata by shortcode
///
/// TODO: Add error handling with correct status codes
pub async fn get_project_metadata_by_shortcode(
    Path(shortcode): Path<String>,
    State(state): State<Arc<AppState>>,
) -> Json<Value> {
    trace!("entered get_project_metadata_by_shortcode()");
    let project_metadata = state
        .project_metadata_service
        .get_by_shortcode(Shortcode(shortcode));
    Json(serde_json::to_value(project_metadata).unwrap())
}

pub async fn get_all_project_metadata(State(state): State<Arc<AppState>>) -> Json<Value> {
    trace!("entered get_all_project_metadata()");
    let all_project_metadata = state.project_metadata_service.get_all();
    Json(serde_json::to_value(all_project_metadata).unwrap())
}

pub async fn store_project_metadata(
    State(state): State<Arc<AppState>>,
    body: String,
) -> Result<(), DspMetaError> {
    trace!("entered store_project_metadata");

    let service = &state.project_metadata_service;

    let hcl_body = hcl::from_str(body.as_str())?;
    let project_metadata = ProjectMetadata::try_from(&hcl_body)?;

    service.store(&project_metadata.project.shortcode, &project_metadata)
}

// basic handler that responds with a static string
pub async fn hello_world(State(state): State<Arc<AppState>>) -> &'static str {
    trace!("entered hello_world()");
    let _ = state.project_metadata_service;
    "Hello, World!"
}

pub async fn get_root() {}

pub async fn post_root() {}

pub async fn foo_bar() {}

pub async fn create_user(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<CreateUser>,
) -> (StatusCode, Json<User>) {
    // insert your application logic here
    let user = User {
        id: 1337,
        username: payload.username,
    };

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(user))
}

// the input to our `create_user` handler
#[derive(Debug, Deserialize)]
pub struct CreateUser {
    username: String,
}

// the output to our `create_user` handler
#[derive(Debug, Serialize)]
pub struct User {
    id: u64,
    username: String,
}
