use tracing::trace;

use crate::domain::model::entity::project_metadata::ProjectMetadata;
use crate::domain::model::value::Shortcode;
use crate::domain::service::project_metadata_api_contract::ProjectMetadataApiContract;
use crate::domain::service::repository_contract::RepositoryContract;
use crate::errors::DspMetaError;

#[derive(Debug, Clone)]
pub struct ProjectMetadataService<Repository> {
    repo: Repository,
}

impl<Repository> ProjectMetadataService<Repository>
where
    Repository: RepositoryContract<ProjectMetadata, Shortcode, DspMetaError>,
{
    pub fn new(repo: Repository) -> Self {
        trace!("Entering ProjectMetadataService::new()");
        Self { repo }
    }
}

impl<R> ProjectMetadataApiContract for ProjectMetadataService<R>
where
    R: RepositoryContract<ProjectMetadata, Shortcode, DspMetaError>,
{
    fn find_by_id(&self, id: Shortcode) -> Result<Option<ProjectMetadata>, DspMetaError> {
        self.repo.find_by_id(&id)
    }

    fn find_all(&self) -> Result<Vec<ProjectMetadata>, DspMetaError> {
        self.repo.find_all()
    }

    fn save(&self, entity: ProjectMetadata) -> Result<ProjectMetadata, DspMetaError> {
        self.repo.save(entity)
    }

    fn delete(&self, id: Shortcode) -> Result<(), DspMetaError> {
        let entity_to_delete = self.repo.find_by_id(&id)?;
        match entity_to_delete {
            Some(entity) => self.repo.delete(entity),
            None => Ok(()),
        }
    }
}
