use dsp_domain::metadata::entity::project_metadata::ProjectMetadata;
use dsp_domain::metadata::value::Shortcode;

use crate::error::DspMetaError;

pub trait ProjectMetadataApiContract {
    fn find_by_id(&self, id: Shortcode) -> Result<Option<ProjectMetadata>, DspMetaError>;
    fn find_all(&self) -> Result<Vec<ProjectMetadata>, DspMetaError>;
    fn save(&self, entity: ProjectMetadata) -> Result<ProjectMetadata, DspMetaError>;
    fn delete(&self, id: Shortcode) -> Result<(), DspMetaError>;
}
