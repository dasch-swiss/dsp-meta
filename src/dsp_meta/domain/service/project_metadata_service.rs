use tracing::trace;

use crate::domain::model::entity::project_metadata::ProjectMetadata;
use crate::domain::model::value::Shortcode;
use crate::domain::service::project_metadata_api_contract::ProjectMetadataApiContract;
use crate::domain::service::project_metadata_repository_contract::ProjectMetadataRepositoryContract;
use crate::errors::DspMetaError;

#[derive(Debug, Clone)]
pub struct ProjectMetadataService<R> {
    repo: R,
}

impl<R> ProjectMetadataService<R>
where
    R: ProjectMetadataRepositoryContract,
{
    pub fn new(repo: R) -> Self {
        trace!("Entering ProjectMetadataService::new()");
        Self { repo }
    }
}

impl<R> ProjectMetadataApiContract for ProjectMetadataService<R>
where
    R: ProjectMetadataRepositoryContract,
{
    fn get_by_shortcode(&self, id: Shortcode) -> Result<ProjectMetadata, DspMetaError> {
        self.repo.get_by_shortcode(id)
    }

    fn get_all(&self) -> Result<Vec<ProjectMetadata>, DspMetaError> {
        self.repo.get_all()
    }

    fn store(&self, id: &Shortcode, metadata: &ProjectMetadata) -> Result<(), DspMetaError> {
        self.repo.store(id, metadata)
    }

    fn delete(&self, id: Shortcode) -> Result<(), DspMetaError> {
        self.repo.delete(id)
    }
}
