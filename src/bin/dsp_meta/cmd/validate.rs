use std::fs;
use std::path::PathBuf;

use dsp_meta::domain::metadata::Metadata;

/// Read projects definition from .toml
pub fn validate(project: &PathBuf) {
    println! {"Hello from validate!"};

    let toml = fs::read_to_string(project).unwrap();
    let _: Metadata = toml::from_str(toml.as_str()).unwrap();
}

#[test]
fn deserialize_metadata_from_toml() {
    use dsp_meta::domain::dataset::Dataset;
    use dsp_meta::domain::metadata::Metadata;
    use dsp_meta::domain::person::Person;
    use dsp_meta::domain::project::Project;

    let metadata: Metadata = Metadata::make(
        Project::make("0000-project", "0000"),
        vec![
            Dataset::make("dataset-001", "Dataset 1"),
            Dataset::make("dataset-002", "Dataset 2"),
        ],
        vec![Person::make("person-001"), Person::make("person-002")],
    );

    let metadata_from_toml: Metadata = toml::from_str(
        r#"
            [project]
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

    assert_eq!(metadata, metadata_from_toml);
}
