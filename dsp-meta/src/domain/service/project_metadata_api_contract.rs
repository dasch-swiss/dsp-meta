use dsp_domain::metadata::value::Shortcode;
use crate::api::convert::serde::draft_model::DraftMetadata;
use crate::domain::service::repository_contract::Pagination;
use crate::error::DspMetaError;

pub trait ProjectMetadataApiContract {
    fn find_by_id(&self, id: Shortcode) -> Result<Option<DraftMetadata>, DspMetaError>;
    fn find_all(&self, pagination: &Pagination) -> Result<Vec<DraftMetadata>, DspMetaError>;
    fn count(&self) -> Result<usize, DspMetaError>;
}
