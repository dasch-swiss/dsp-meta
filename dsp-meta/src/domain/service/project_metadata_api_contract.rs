use dsp_domain::metadata::entity::project_metadata::ProjectMetadata;
use dsp_domain::metadata::value::Shortcode;

use crate::domain::model::project_info::ProjectInfo;
use crate::error::DspMetaError;

pub trait ProjectMetadataApiContract {
    fn find_by_id(&self, id: Shortcode) -> Result<Option<ProjectMetadata>, DspMetaError>;
    fn find_all(&self) -> Result<Vec<ProjectInfo>, DspMetaError>;
    fn count(&self) -> Result<usize, DspMetaError>;
}
