use std::sync::Arc;

use axum::extract::State;
use axum::http::StatusCode;
use axum::response::Json;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tracing::trace;

use crate::app_state::app_state::AppState;
use crate::domain::value::Shortcode;
use crate::service::project_metadata_api_contract::ProjectMetadataApiContract;

/// GET /project_metadata/:shortcode
/// Get project metadata by shortcode
///
/// TODO: Add error handling with correct status codes
/// TODO: Add parameter extraction
pub async fn get_project_metadata_by_shortcode(
    shortcode: String,
    State(state): State<Arc<AppState>>,
) -> Json<Value> {
    trace!("entered dsp_meta::api::get_project_metadata_by_shortcode()");
    let project_metadata = state
        .project_metadata_service
        .get_by_shortcode(Shortcode(shortcode));
    Json(serde_json::to_value(project_metadata).unwrap())
}

// basic handler that responds with a static string
pub async fn hello_world(State(state): State<Arc<AppState>>) -> &'static str {
    trace!("entered dsp_meta::api::hello_world()");
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
#[derive(Deserialize)]
pub struct CreateUser {
    username: String,
}

// the output to our `create_user` handler
#[derive(Serialize)]
pub struct User {
    id: u64,
    username: String,
}
