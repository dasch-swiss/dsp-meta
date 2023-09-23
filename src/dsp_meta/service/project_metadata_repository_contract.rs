use crate::domain::entity::project_metadata::ProjectMetadata;
use crate::domain::value::Shortcode;
use crate::errors::DspMetaError;

pub trait ProjectMetadataRepositoryContract {
    fn get_by_shortcode(&self, shortcode: Shortcode) -> Result<ProjectMetadata, DspMetaError>;
    fn get_all(&self) -> Result<Vec<ProjectMetadata>, DspMetaError>;
    fn store(&self, shortcode: Shortcode, metadata: ProjectMetadata) -> Result<(), DspMetaError>;
    fn delete(&self, shortcode: Shortcode) -> Result<(), DspMetaError>;
}
