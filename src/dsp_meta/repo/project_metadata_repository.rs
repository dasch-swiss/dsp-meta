use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use tracing::trace;

use crate::domain::entity::project_metadata::ProjectMetadata;
use crate::domain::value::Shortcode;
use crate::errors::DspMetaError;
use crate::service::project_metadata_repository_contract::ProjectMetadataRepositoryContract;

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

impl ProjectMetadataRepositoryContract for ProjectMetadataRepository {
    fn get_by_shortcode(&self, shortcode: Shortcode) -> Result<ProjectMetadata, DspMetaError> {
        let db = self.db.read().unwrap();
        match db.get(shortcode.0.as_str()) {
            Some(metadata) => Ok(metadata.clone()),
            None => Err(DspMetaError::NotFound),
        }
    }

    fn get_all(&self) -> Result<Vec<ProjectMetadata>, DspMetaError> {
        let mut result: Vec<ProjectMetadata> = vec![];
        let db = self.db.read().unwrap();

        for project_metadata in db.values() {
            result.push(project_metadata.clone());
        }

        Ok(result)
    }

    fn store(&self, shortcode: &Shortcode, metadata: &ProjectMetadata) -> Result<(), DspMetaError> {
        let mut db = self.db.write().unwrap();
        db.insert(shortcode.0.to_owned(), metadata.clone());
        Ok(())
    }

    fn delete(&self, shortcode: Shortcode) -> Result<(), DspMetaError> {
        let mut db = self.db.write().unwrap();

        match db.remove(shortcode.0.as_str()) {
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
        let shortcode = Shortcode("1234".to_owned());

        let repo = ProjectMetadataRepository::new();
        let result = repo.store(shortcode, metadata);
        assert!(result.is_ok());
    }
}
