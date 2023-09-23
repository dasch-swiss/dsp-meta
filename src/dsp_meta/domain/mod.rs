mod convert;
pub mod entity;
pub mod value;

#[cfg(test)]
mod tests {
    use hcl::body;

    use crate::domain::entity::project_metadata::ProjectMetadata;

    #[test]
    fn try_from_multiple_projects_error() {
        let input = body!(
            project {
                shortcode = "0803"
            }
            project {
                shortcode = "0804"
            }
        );

        let project = ProjectMetadata::try_from(&input);
        assert!(project.is_err());
    }

    #[test]
    fn try_from_no_project_error() {
        let input = body!();

        let project = ProjectMetadata::try_from(&input);
        assert!(project.is_err());
    }
}
