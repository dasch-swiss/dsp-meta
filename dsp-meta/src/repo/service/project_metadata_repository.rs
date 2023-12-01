use std::collections::HashMap;
use std::path::Path;
use std::sync::{Arc, RwLock};

use dsp_domain::metadata::entity::project_metadata::ProjectMetadata;
use dsp_domain::metadata::value::Shortcode;
use tracing::trace;

use crate::api::convert::hcl::hcl_body::HclBody;
use crate::domain::service::repository_contract::RepositoryContract;
use crate::error::DspMetaError;

#[derive(Debug, Default, Clone)]
pub struct ProjectMetadataRepository {
    db: Arc<RwLock<HashMap<String, ProjectMetadata>>>,
}

impl ProjectMetadataRepository {
    pub fn new<P: AsRef<Path>>(data_path: &P) -> Self {
        trace!("Entering ProjectMetadataRepository::new");
        let db: Arc<RwLock<HashMap<String, ProjectMetadata>>> =
            Arc::new(RwLock::new(HashMap::new()));

        // get paths of HCL files
        let hcl_files = std::fs::read_dir(data_path)
            .expect("read directory containing HCL files.")
            .map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<_>, std::io::Error>>()
            .expect("collect all files into collection.");

        // load into db
        for file in hcl_files {
            let input = std::fs::read_to_string(file).expect("read file.");
            let body: hcl::Body = hcl::from_str(&input).expect("parse file as HCL body.");
            let entity: ProjectMetadata = HclBody(&body)
                .try_into()
                .expect("convert file into project metadata.");
            let mut db = db.write().unwrap();
            db.insert(entity.project.shortcode.0.to_owned(), entity.clone());
        }

        Self { db }
    }

    fn _save(&self, entity: ProjectMetadata) -> Result<ProjectMetadata, DspMetaError> {
        let mut db = self.db.write().unwrap();
        db.insert(entity.project.shortcode.0.to_owned(), entity.clone());
        Ok(entity)
    }

    fn _delete(&self, entity: ProjectMetadata) -> Result<(), DspMetaError> {
        let mut db = self.db.write().unwrap();

        match db.remove(entity.project.shortcode.0.as_str()) {
            Some(_) => Ok(()),
            None => Ok(()),
        }
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
