use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use dsp_domain::metadata::entity::project_metadata::ProjectMetadata;
use dsp_domain::metadata::value::Shortcode;
use tracing::trace;

use crate::domain::service::repository_contract::RepositoryContract;
use crate::error::DspMetaError;

#[derive(Debug, Default, Clone)]
pub struct ProjectMetadataRepository {
    db: Arc<RwLock<HashMap<String, ProjectMetadata>>>,
}

impl ProjectMetadataRepository {
    pub fn new() -> Self {
        trace!("Entering ProjectMetadataRepository::new");
        let db: Arc<RwLock<HashMap<String, ProjectMetadata>>> =
            Arc::new(RwLock::new(HashMap::new()));
        Self { db }
    }
}

impl RepositoryContract<ProjectMetadata, Shortcode, DspMetaError> for ProjectMetadataRepository {
    fn find_by_id(&self, id: &Shortcode) -> Result<Option<ProjectMetadata>, DspMetaError> {
        let db = self.db.read().unwrap();
        match db.get(id.0.as_str()) {
            Some(metadata) => Ok(Some(metadata.clone())),
            None => Ok(None),
        }
    }

    fn find_all(&self) -> Result<Vec<ProjectMetadata>, DspMetaError> {
        let mut result: Vec<ProjectMetadata> = vec![];
        let db = self.db.read().unwrap();

        for project_metadata in db.values() {
            result.push(project_metadata.clone());
        }

        Ok(result)
    }

    fn save(&self, entity: ProjectMetadata) -> Result<ProjectMetadata, DspMetaError> {
        let mut db = self.db.write().unwrap();
        db.insert(entity.project.shortcode.0.to_owned(), entity.clone());
        Ok(entity)
    }

    fn delete(&self, entity: ProjectMetadata) -> Result<(), DspMetaError> {
        let mut db = self.db.write().unwrap();

        match db.remove(entity.project.shortcode.0.as_str()) {
            Some(_) => Ok(()),
            None => Ok(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn successfully_store_project_metadata() {
        let metadata = ProjectMetadata::default();

        let repo = ProjectMetadataRepository::new();
        let result = repo.save(metadata);
        assert!(result.is_ok());
    }
}
