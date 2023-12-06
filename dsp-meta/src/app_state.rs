use crate::domain::service::project_metadata_service::ProjectMetadataService;
use crate::repo::service::project_metadata_repository::ProjectMetadataRepository;

#[derive(Debug, Clone)]
pub struct AppState {
    pub project_metadata_service: ProjectMetadataService<ProjectMetadataRepository>,
    pub frontend_dir: String,
    pub version: &'static str,
}
