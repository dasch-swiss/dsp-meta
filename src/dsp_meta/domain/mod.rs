mod convert;
pub(crate) mod entity;
mod value;

#[cfg(test)]
mod tests {
    use crate::domain::entity::metadata::Metadata;
    use hcl::body;

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

        let project = Metadata::try_from(&input);
        assert!(project.is_err());
    }

    #[test]
    fn try_from_no_project_error() {
        let input = body!();

        let project = Metadata::try_from(&input);
        assert!(project.is_err());
    }
}
