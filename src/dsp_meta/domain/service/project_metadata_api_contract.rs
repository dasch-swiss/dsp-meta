use crate::domain::model::entity::project_metadata::ProjectMetadata;
use crate::domain::model::value::Shortcode;
use crate::error::DspMetaError;

pub trait ProjectMetadataApiContract {
    fn find_by_id(&self, id: Shortcode) -> Result<Option<ProjectMetadata>, DspMetaError>;
    fn find_all(&self) -> Result<Vec<ProjectMetadata>, DspMetaError>;
    fn save(&self, entity: ProjectMetadata) -> Result<ProjectMetadata, DspMetaError>;
    fn delete(&self, id: Shortcode) -> Result<(), DspMetaError>;
}
