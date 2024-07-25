use std::collections::HashMap;
use std::fs::File;
use std::path::Path;
use std::sync::{Arc, RwLock};

use dsp_domain::metadata::value::Shortcode;
use tracing::{instrument, trace};

use crate::api::convert::serde::draft_model::{DraftMetadata, DraftProjectStatus};
use crate::domain::service::repository_contract::{Filter, Page, Pagination, RepositoryContract};
use crate::error::DspMetaError;
use crate::infrastructure::load_json_file_paths;

#[derive(Debug, Default, Clone)]
pub struct ProjectMetadataRepository {
    db: Arc<RwLock<HashMap<String, DraftMetadata>>>,
}

impl ProjectMetadataRepository {
    pub fn new(data_path: &Path) -> Self {
        trace!("Init Repository {:?}", data_path);
        let db: Arc<RwLock<HashMap<String, DraftMetadata>>> = Arc::new(RwLock::new(HashMap::new()));

        let file_paths = load_json_file_paths(data_path);
        trace!("Found {} projects", file_paths.len());

        let mut known_shortcodes: Vec<String> = Vec::new();
        for file in file_paths {
            let file = File::open(file).expect("open file.");
            let entity: DraftMetadata = serde_json::from_reader(file).expect("parse file as JSON.");
            let mut db = db.write().unwrap();
            let shortcode = entity.project.shortcode.to_owned();
            if known_shortcodes.contains(&shortcode) {
                panic!("Duplicate shortcode: {}", shortcode);
            }
            known_shortcodes.push(shortcode);

            db.insert(entity.project.shortcode.to_owned(), entity);
        }

        {
            let count = db.read().unwrap();
            trace!("Stored {} projects", count.values().len());
        }

        Self { db }
    }
}

impl RepositoryContract<DraftMetadata, Shortcode, DspMetaError> for ProjectMetadataRepository {
    #[instrument(skip(self))]
    fn find_by_id(&self, id: &Shortcode) -> Result<Option<DraftMetadata>, DspMetaError> {
        let db = self.db.read().unwrap();
        match db.get(id.0.as_str()) {
            Some(metadata) => Ok(Some(metadata.clone())),
            None => Ok(None),
        }
    }

    #[instrument(skip(self))]
    fn find(
        &self,
        filter: &Filter,
        pagination: &Pagination,
    ) -> Result<Page<DraftMetadata>, DspMetaError> {
        let db = self.db.read().unwrap();
        let query_status: Option<Vec<DraftProjectStatus>> = match filter.filter.as_deref() {
            Some("o") => Some(vec![DraftProjectStatus::Ongoing]),
            Some("f") => Some(vec![DraftProjectStatus::Finished]),
            Some("of") => Some(vec![
                DraftProjectStatus::Ongoing,
                DraftProjectStatus::Finished,
            ]),
            _ => None,
        };

        let values = db
            .values()
            .filter(|metadata| {
                if let Some(query_status) = &query_status {
                    let actual_status = &metadata.project.status.clone().unwrap_or_default();
                    !query_status.contains(actual_status)
                } else {
                    true
                }
            })
            .filter(|metadata| {
                if let Some(query) = &filter.query {
                    serde_json::to_string(metadata).unwrap().contains(query)
                } else {
                    true
                }
            }).cloned().collect::<Vec<DraftMetadata>>();
        let total = values.len();
        let data = values
            .into_iter()
            .skip((pagination.page - 1) * pagination.limit)
            .take(pagination.limit)
            .collect::<Vec<DraftMetadata>>();
        Ok(Page { data, total })
    }

    fn count(&self) -> Result<usize, DspMetaError> {
        let db = self.db.read().unwrap();
        Ok(db.len())
    }
}

#[cfg(test)]
mod tests {
    use std::env;

    use super::*;

    #[test]
    fn successfully_load_metadata() {
        let data_dir = env::current_dir().unwrap().parent().unwrap().join("data");
        dbg!(&data_dir);

        let files = load_json_file_paths(&data_dir);
        let repo = ProjectMetadataRepository::new(&data_dir.as_path());
        let actual = repo.count().unwrap();
        assert_eq!(actual, files.len());
    }
}
