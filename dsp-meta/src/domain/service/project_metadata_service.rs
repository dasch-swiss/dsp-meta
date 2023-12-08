use dsp_domain::metadata::entity::project_metadata::ProjectMetadata;
use dsp_domain::metadata::value::Shortcode;
use tracing::{instrument, trace};

use crate::domain::model::project_info::ProjectInfo;
use crate::domain::service::project_metadata_api_contract::ProjectMetadataApiContract;
use crate::domain::service::repository_contract::RepositoryContract;
use crate::error::DspMetaError;

#[derive(Debug, Clone)]
pub struct ProjectMetadataService<Repository> {
    repo: Repository,
}

impl<Repository> ProjectMetadataService<Repository>
where
    Repository: RepositoryContract<ProjectMetadata, ProjectInfo, Shortcode, DspMetaError>,
{
    pub fn new(repo: Repository) -> Self {
        trace!("Init Service");
        Self { repo }
    }
}

impl<R> ProjectMetadataApiContract for ProjectMetadataService<R>
where
    R: RepositoryContract<ProjectMetadata, ProjectInfo, Shortcode, DspMetaError> + std::fmt::Debug,
{
    fn find_by_id(&self, id: Shortcode) -> Result<Option<ProjectMetadata>, DspMetaError> {
        self.repo.find_by_id(&id)
    }

    #[instrument(skip(self))]
    fn find_all(&self) -> Result<Vec<ProjectInfo>, DspMetaError> {
        trace!("service: find_all");
        self.repo.find_all()
    }

    fn count(&self) -> Result<usize, DspMetaError> {
        self.repo.count()
    }
}
