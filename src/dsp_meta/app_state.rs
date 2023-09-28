use crate::domain::service::project_metadata_service::ProjectMetadataService;
use crate::repo::project_metadata_repository::ProjectMetadataRepository;

#[derive(Debug, Clone)]
pub struct AppState {
    pub project_metadata_service: ProjectMetadataService<ProjectMetadataRepository>,
}
