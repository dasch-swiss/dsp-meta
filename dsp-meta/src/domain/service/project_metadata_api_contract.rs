use crate::domain::model::draft_model::*;
use crate::domain::service::repository_contract::{Filter, Page, Pagination};
use crate::error::DspMetaError;

pub trait ProjectMetadataApiContract {
    fn find_by_id(&self, id: &Shortcode) -> Result<Option<DraftMetadata>, DspMetaError>;
    fn find_all(&self) -> Result<Vec<DraftMetadata>, DspMetaError>;
    fn find(
        &self,
        filter: &Filter,
        pagination: &Pagination,
    ) -> Result<Page<DraftMetadata>, DspMetaError>;
}
