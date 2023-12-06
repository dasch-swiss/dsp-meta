use dsp_domain::metadata::entity::project::Project;
use dsp_domain::metadata::value::status::Status;
use dsp_domain::metadata::value::{Name, Shortcode, TeaserText};
use serde::Serialize;

#[derive(Debug, Default, Clone, PartialEq, Serialize)]
pub struct ProjectInfo {
    id: Shortcode,
    name: Name,
    description: TeaserText,
    status: Status,
}

impl From<Project> for ProjectInfo {
    fn from(value: Project) -> Self {
        ProjectInfo {
            id: value.shortcode,
            name: value.name,
            description: value.teaser_text,
            status: value.status,
        }
    }
}
