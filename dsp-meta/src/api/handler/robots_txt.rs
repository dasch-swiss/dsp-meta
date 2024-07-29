use std::sync::Arc;

use axum::extract::State;
use axum::http::{Response, StatusCode};

use crate::app_state::AppState;
use crate::error::DspMetaError;

pub async fn robots_txt(
    State(state): State<Arc<AppState>>,
) -> Result<Response<String>, DspMetaError> {
    let sitemap_xml = state.base_url.join( "sitemap.xml").expect("valid url").to_string();
    let response = Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/plain")
        .body(format!(
            "Sitemap: {}\nUser-agent: *\nDisallow:\n",
            sitemap_xml
        ))
        .expect("Failed to build response");
    Ok(response)
}
