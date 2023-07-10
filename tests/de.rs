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
        .add_block(
            Block::builder("project")
                .add_label("0803")
                .add_attribute(("created_at", "1637624150548721000"))
                .add_attribute(("created_by", "dsp-metadata-gui"))
                .add_attribute(("shortcode", "0803"))
                .add_attribute(("teaser_text", "An artscientific monograph of the richly illustrated early prints in Basel"))
                .add_attribute(("start_date", "2008-06-01"))
                .add_attribute(("end_date", "2012-08-31"))
                .add_block(
                    Block::builder("keyword")
                        .add_attribute(("en", "local history"))
                        .add_attribute(("de", "Lokalgeschichte"))
                        .build(),
                )
                .add_block(
                    Block::builder("keyword")
                        .add_attribute(("en", "regional history"))
                        .add_attribute(("de", "Regionalgeschichte"))
                        .build(),
                )
                .add_block(
                    Block::builder("name")
                        .add_label("1")
                        .add_attribute(("de", "Die Bilderfolgen der Basler Frühdrucke: Spätmittelalterliche Didaxe als Bild-Text-Lektüre"))
                        .build(),
                )
                .add_block(
                    Block::builder("name")
                        .add_label("2")
                        .add_attribute(("en", "Incunabula"))
                        .build(),
                )
                .add_block(
                    Block::builder("discipline")
                        .add_block(
                            Block::builder("text")
                                .add_attribute(("en", "10404 Visual arts and Art history"))
                                .build(),
                        )
                        .build()
                )
                .build()
        )
        .build();

    let input = r#"
          project "0803" {
            created_at  = "1637624150548721000"
            created_by  = "dsp-metadata-gui"
            shortcode   = "0803"
            teaser_text = "An artscientific monograph of the richly illustrated early prints in Basel"
            start_date  = "2008-06-01"
            end_date    = "2012-08-31"

            keyword {
              en = "local history"
              de = "Lokalgeschichte"
            }
            keyword {
              en = "regional history"
              de = "Regionalgeschichte"
            }

            name "1" {
              de = "Die Bilderfolgen der Basler Frühdrucke: Spätmittelalterliche Didaxe als Bild-Text-Lektüre"
            }
            name "2" {
              en = "Incunabula"
            }

            discipline {
              text {
                en = "10404 Visual arts and Art history"
              }
            }
          }
    "#;
    let actual: Body = hcl::from_str(input).unwrap();

    assert_eq!(expected, actual);
}
