use tracing::{instrument, trace};

use crate::domain::metadata_repository::{Filter, MetadataRepository, Page, Pagination};
use crate::domain::model::draft_model::*;
use crate::error::DspMetaError;

#[derive(Debug, Clone)]
pub struct MetadataService {
    repo: MetadataRepository,
}

impl MetadataService {
    pub fn new(repo: MetadataRepository) -> Self {
        trace!("Init MetadataService");
        Self { repo }
    }

    pub fn find_by_id(&self, id: &Shortcode) -> Result<Option<DraftMetadata>, DspMetaError> {
        self.repo.find_by_id(id)
    }

    pub fn find_all(&self) -> Result<Vec<DraftMetadata>, DspMetaError> {
        self.repo.find_all()
    }

    #[instrument(skip(self))]
    pub fn find(
        &self,
        filter: &Filter,
        pagination: &Pagination,
    ) -> Result<Page<DraftMetadata>, DspMetaError> {
        self.repo.find(filter, pagination)
    }
}
