use tracing::trace;

use crate::domain::entity::project_metadata::ProjectMetadata;
use crate::domain::value::Shortcode;
use crate::errors::DspMetaError;
use crate::service::project_metadata_api_contract::ProjectMetadataApiContract;
use crate::service::project_metadata_repository_contract::ProjectMetadataRepositoryContract;

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
        todo!()
    }

    fn store(&self, id: Shortcode, metadata: ProjectMetadata) -> Result<(), DspMetaError> {
        self.repo.store(id, metadata)
    }

    fn delete(&self, id: Shortcode) -> Result<(), DspMetaError> {
        self.repo.delete(id)
    }
}
