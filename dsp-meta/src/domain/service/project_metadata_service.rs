use dsp_domain::metadata::value::Shortcode;
use tracing::{instrument, trace};

use crate::api::convert::serde::draft_model::DraftMetadata;
use crate::domain::service::project_metadata_api_contract::ProjectMetadataApiContract;
use crate::domain::service::repository_contract::{Filter, Page, Pagination, RepositoryContract};
use crate::error::DspMetaError;

#[derive(Debug, Clone)]
pub struct ProjectMetadataService<Repository> {
    repo: Repository,
}

impl<Repository> ProjectMetadataService<Repository>
where
    Repository: RepositoryContract<DraftMetadata, Shortcode, DspMetaError>,
{
    pub fn new(repo: Repository) -> Self {
        trace!("Init Service");
        Self { repo }
    }
}

impl<R> ProjectMetadataApiContract for ProjectMetadataService<R>
where
    R: RepositoryContract<DraftMetadata, Shortcode, DspMetaError> + std::fmt::Debug,
{
    fn find_by_id(&self, id: Shortcode) -> Result<Option<DraftMetadata>, DspMetaError> {
        self.repo.find_by_id(&id)
    }

    #[instrument(skip(self))]
    fn find(
        &self,
        filter: &Filter,
        pagination: &Pagination,
    ) -> Result<Page<DraftMetadata>, DspMetaError> {
        self.repo.find(filter, pagination)
    }
}
