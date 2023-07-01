use std::fs;
use std::path::PathBuf;

use crate::domain::project::Project;

/// Read projects definition from .toml
pub fn validate(project: &PathBuf) -> anyhow::Result<()> {
    println! {"Hello from validate!"};

    // TODO: needs better error message in case that the path cannot be opened.
    let toml = fs::read_to_string(project)?;
    println!("read project: \n{toml}");
    let _: Project = toml::from_str(toml.as_str())?;
    anyhow::Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn deserialize_metadata_from_toml() {
        use crate::domain::dataset::Dataset;
        use crate::domain::person::Person;
        use crate::domain::project::Project;

        let project_metadata: Project = Project::make(
            "0000-project",
            "0000",
            vec![
                Dataset::make("dataset-001", "Dataset 1"),
                Dataset::make("dataset-002", "Dataset 2"),
            ],
            vec![Person::make("person-001"), Person::make("person-002")],
        );

        let project_metadata_from_toml: Project = toml::from_str(
            r#"
            id = "0000-project"
            shortcode = "0000"

            [[datasets]]
            id = "dataset-001"
            title = "Dataset 1"

            [[datasets]]
            id = "dataset-002"
            title = "Dataset 2"

            [[persons]]
            id = "person-001"

            [[persons]]
            id = "person-002"
        "#,
        )
        .unwrap();

        assert_eq!(project_metadata, project_metadata_from_toml);
    }
}
