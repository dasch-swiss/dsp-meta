use tracing::trace;

use crate::repo::project_metadata_repository::ProjectMetadataRepository;
use crate::service::project_metadata_service::ProjectMetadataService;

pub fn run() {
    trace!("tracing from dsp_meta::core::server::run()");
    let _project_metadata_service = ProjectMetadataService::new(ProjectMetadataRepository::new());
}
