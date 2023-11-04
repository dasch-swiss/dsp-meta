pub mod api;
pub mod app_state;
pub mod domain;
pub mod errors;
pub mod operation;
pub mod repo;

#[cfg(test)]
mod tests {
    use hcl::body;

    #[test]
    fn deserialize_metadata_from_hcl() {
        use hcl::{Block, Body};

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

        let actual = body!(
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
        );
        assert_eq!(expected, actual);
    }

    #[test]
    fn deserialize_hcl_block_label() {
        use hcl::{Attribute, Block};

        let body = body!(
              version = 1
              project "0803" {
                shortcode = "0803"
              }
              dataset "dataset-001" {
                title = "One dataset"
              }
              dataset "dataset-002" {
                title = "Another dataset"
              }
        );
        dbg!(&body);

        let attributes: Vec<&Attribute> = body.attributes().collect();
        dbg!(attributes);

        let blocks: Vec<&Block> = body.blocks().collect();
        dbg!(blocks);

        let json: serde_json::Value = hcl::from_body(body).unwrap();
        dbg!(json);
    }
}
