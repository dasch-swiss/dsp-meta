mod common;

use common::init;
use dsp_meta::domain::dataset::Dataset;
use dsp_meta::domain::person::Person;
use dsp_meta::domain::project::Project;

#[test]
fn deserialize_metadata_from_toml() {
    init();

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
