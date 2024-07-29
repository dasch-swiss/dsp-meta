use url::Url;

use crate::domain::service::metadata_service::MetadataService;

#[derive(Debug, Clone)]
pub struct AppState {
    pub project_metadata_service: MetadataService,
    pub public_dir: String,
    pub version: &'static str,
    pub base_url: Url,
}
