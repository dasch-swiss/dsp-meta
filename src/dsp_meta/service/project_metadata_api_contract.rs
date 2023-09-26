use crate::domain::entity::project_metadata::ProjectMetadata;
use crate::domain::value::Shortcode;
use crate::errors::DspMetaError;

pub trait ProjectMetadataApiContract {
    fn get_by_shortcode(&self, id: Shortcode) -> Result<ProjectMetadata, DspMetaError>;
    fn get_all(&self) -> Result<Vec<ProjectMetadata>, DspMetaError>;
    fn store(&self, id: &Shortcode, metadata: &ProjectMetadata) -> Result<(), DspMetaError>;
    fn delete(&self, id: Shortcode) -> Result<(), DspMetaError>;
}
