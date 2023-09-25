use crate::repo::project_metadata_repository::ProjectMetadataRepository;
use crate::service::project_metadata_service::ProjectMetadataService;

#[derive(Debug, Clone)]
pub struct AppState {
    pub project_metadata_service: ProjectMetadataService<ProjectMetadataRepository>,
}
