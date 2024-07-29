use std::collections::HashMap;
use std::fs::File;
use std::path::Path;
use std::sync::{Arc, RwLock};

use log::info;
use serde::Deserialize;
use tracing::{instrument, trace};

use crate::domain::model::draft_model::*;
use crate::error::DspMetaError;
use crate::infrastructure::load_json_file_paths;

#[derive(Deserialize, Debug)]
pub struct Pagination {
    #[serde(rename = "_page")]
    pub page: usize,
    #[serde(rename = "_limit")]
    pub limit: usize,
}
impl Default for Pagination {
    fn default() -> Self {
        Pagination { page: 1, limit: 10 }
    }
}

#[derive(Deserialize, Default, Debug)]
pub struct Filter {
    #[serde(rename = "q")]
    pub query: Option<String>,
    #[serde(rename = "filter")]
    pub filter: Option<String>,
}

pub struct Page<T> {
    pub data: Vec<T>,
    pub total: usize,
}

#[derive(Debug, Default, Clone)]
pub struct MetadataRepository {
    db: Arc<RwLock<HashMap<Shortcode, DraftMetadata>>>,
}

impl MetadataRepository {
    pub fn for_test(metadata: Vec<DraftMetadata>) -> Self {
        let db: Arc<RwLock<HashMap<Shortcode, DraftMetadata>>> =
            Arc::new(RwLock::new(HashMap::new()));

        for meta in metadata {
            let mut db = db.write().unwrap();
            db.insert(meta.project.shortcode.to_owned(), meta);
        }

        Self { db }
    }
    pub fn from_path(data_path: &Path) -> Self {
        info!("Init Repository {:?}", data_path);
        let db: Arc<RwLock<HashMap<Shortcode, DraftMetadata>>> =
            Arc::new(RwLock::new(HashMap::new()));

        let file_paths = load_json_file_paths(data_path);
        info!("Found {} projects", file_paths.len());

        let mut known_shortcodes: Vec<Shortcode> = Vec::new();
        for file in file_paths {
            let file = File::open(file).expect("open file.");
            let entity: DraftMetadata = serde_json::from_reader(file).expect("parse file as JSON.");
            let mut db = db.write().unwrap();
            let shortcode = entity.project.shortcode.to_owned();
            if known_shortcodes.contains(&shortcode) {
                panic!("Duplicate shortcode: {:?}", shortcode);
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

    #[instrument(skip(self))]
    pub fn find_by_id(&self, id: &Shortcode) -> Result<Option<DraftMetadata>, DspMetaError> {
        let db = self.db.read().unwrap();
        match db.get(id) {
            Some(metadata) => Ok(Some(metadata.clone())),
            None => Ok(None),
        }
    }

    #[instrument(skip(self))]
    pub fn find(
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
                    serde_json::to_string(metadata)
                        .unwrap()
                        .to_lowercase()
                        .contains(&query.to_lowercase())
                } else {
                    true
                }
            })
            .cloned()
            .collect::<Vec<DraftMetadata>>();
        let total = values.len();
        let data = values
            .into_iter()
            .skip((pagination.page - 1) * pagination.limit)
            .take(pagination.limit)
            .collect::<Vec<DraftMetadata>>();
        Ok(Page { data, total })
    }

    pub fn find_all(&self) -> Result<Vec<DraftMetadata>, DspMetaError> {
        let db = self.db.read().unwrap();
        let v = db.iter().map(|(_, v)| v.clone()).collect();
        Ok(v)
    }

    pub fn count(&self) -> Result<usize, DspMetaError> {
        let db = self.db.read().unwrap();
        Ok(db.len())
    }
}

#[cfg(test)]
mod tests {
    use std::env;

    use super::*;

    #[test]
    fn successfully_load_all_metadata_files() {
        let data_dir = env::current_dir().unwrap().parent().unwrap().join("data");
        dbg!(&data_dir);

        let files = load_json_file_paths(&data_dir);
        let repo = MetadataRepository::from_path(&data_dir.as_path());
        let actual = repo.count().expect("count");
        assert_eq!(actual, files.len());
    }
}
