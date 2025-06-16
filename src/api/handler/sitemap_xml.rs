use std::sync::Arc;

use axum::extract::State;
use axum::http::StatusCode;
use axum::response::Response;
use tracing::instrument;

use crate::app_state::AppState;
use crate::error::DspMetaError;

#[instrument(skip(state))]
pub async fn sitemap_xml(
    State(state): State<Arc<AppState>>,
) -> Result<Response<String>, DspMetaError> {
    let base_url = state.base_url.clone();
    let mut xml = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
    xml.push_str("<urlset xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\">\n");
    xml.push_str(
        format!("<url><loc>{base_url}</loc><changefreq>weekly</changefreq></url>\n",).as_str(),
    );
    for meta in state.metadata_service.find_all()? {
        let mut url = base_url.to_string() + "projects/";
        url.push_str(&meta.project.shortcode.as_string());
        let line = format!("<url><loc>{url}</loc><changefreq>weekly</changefreq></url>\n",);
        xml.push_str(line.as_str());
    }
    xml.push_str("</urlset>\n");

    let resp = Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/xml")
        .body(xml)
        .expect("Failed to build response");
    Ok(resp)
}
