use std::collections::HashMap;
use std::path::Path;
use std::sync::{Arc, RwLock};

use dsp_domain::metadata::entity::project_metadata::ProjectMetadata;
use dsp_domain::metadata::value::Shortcode;
use tracing::{instrument, trace};

use crate::api::convert::hcl::hcl_body::HclBody;
use crate::domain::service::repository_contract::{Pagination, RepositoryContract};
use crate::error::DspMetaError;
use crate::infrastructure::load_hcl_file_paths;

#[derive(Debug, Default, Clone)]
pub struct ProjectMetadataRepository {
    db: Arc<RwLock<HashMap<String, ProjectMetadata>>>,
}

impl ProjectMetadataRepository {
    pub fn new(data_path: &Path) -> Self {
        trace!("Init Repository");
        let db: Arc<RwLock<HashMap<String, ProjectMetadata>>> =
            Arc::new(RwLock::new(HashMap::new()));

        for file in load_hcl_file_paths(data_path) {
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
}

impl RepositoryContract<ProjectMetadata, Shortcode, DspMetaError> for ProjectMetadataRepository {
    #[instrument(skip(self))]
    fn find_by_id(&self, id: &Shortcode) -> Result<Option<ProjectMetadata>, DspMetaError> {
        let db = self.db.read().unwrap();
        match db.get(id.0.as_str()) {
            Some(metadata) => Ok(Some(metadata.clone())),
            None => Ok(None),
        }
    }

    #[instrument(skip(self))]
    fn find(&self, pagination: &Pagination) -> Result<Vec<ProjectMetadata>, DspMetaError> {
        trace!("repository: find_all");
        let db = self.db.read().unwrap();
        let values = db
            .values()
            .skip((pagination.page - 1) * pagination.limit)
            .take(pagination.limit);
        Ok(values.cloned().collect())
    }
}

#[cfg(test)]
mod tests {
    use std::env;

    use super::*;

    #[test]
    fn successfully_store_project_metadata() {
        let data_dir = env::current_dir().unwrap().parent().unwrap().join("data");
        dbg!(&data_dir);

        let repo = ProjectMetadataRepository::new(&data_dir.as_path());
        let pagination = Pagination::default();
        let result = repo.find(&pagination).unwrap();
        assert_eq!(result.len(), 3);
    }
}
