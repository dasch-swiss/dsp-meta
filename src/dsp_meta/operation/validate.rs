use std::fs;
use std::path::PathBuf;

use log::{error, info, trace, warn};
use toml::Value::String;

use crate::domain::project::Project;

/// Read projects definition from .toml
pub fn validate(project_path: &PathBuf) -> anyhow::Result<()> {
    info!("Hello from validate!");

    let toml = match fs::read_to_string(project_path) {
        Ok(s) => {
            trace!("Successfully read file at: {}", project_path.display());
            anyhow::Ok(s)
        }
        Err(e) => {
            error!(
                "Could not read the file at the supplied path: {}",
                e.to_string()
            );
            Err(anyhow::anyhow!(e))
        }
    }?;
    trace!("read project: \n{toml}");
    let _: Project = toml::from_str(toml.as_str())?;
    anyhow::Ok(())
}

#[cfg(test)]
mod tests {
    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn deserialize_metadata_from_toml() {
        init();

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

        let input = r#"
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
        "#;
        let project_metadata_from_toml: Project = toml::from_str(input).unwrap();

        assert_eq!(project_metadata, project_metadata_from_toml);
    }

    #[test]
    fn deserialize_metadata_from_hcl() {
        use hcl::{Block, Body, Expression};

        use crate::domain::dataset::Dataset;
        use crate::domain::person::Person;
        use crate::domain::project::Project;

        let expected = Body::builder()
            .add_attribute((
                "some_attr",
                Expression::from_iter([
                    ("foo", Expression::from(vec![1, 2])),
                    ("bar", Expression::Bool(true)),
                ]),
            ))
            .add_block(
                Block::builder("some_block")
                    .add_label("some_block_label")
                    .add_attribute(("attr", "value"))
                    .build(),
            )
            .build();

        let input = r#"
            id = "0000-project"
            shortcode = "0000"

            datasets = [
                {
                    id = "dataset-001"
                    title = "Dataset 1"
                    title2 = "Dataset 2"
                },
                {
                   id = "dataset-002"
                   title = "Dataset 2"
                }
            ]

            persons = [
                { id = "person-001" },
                { id = "person-002" }
            ]
        "#;
        let project_metadata_from_hcl: Body = hcl::from_str(input).unwrap();

        assert_eq!(expected, project_metadata_from_hcl);
    }
}
