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
            Block::builder("project")
                .add_label("0803")
                .add_attribute(("created_at", "1637624150548721000"))
                .add_attribute(("created_by", "dsp-metadata-gui"))
                .add_attribute(("shortcode", "0803"))
                .add_attribute(("teaser_text", "An art-scientific monograph of the richly illustrated early prints in Basel - the most important center of early letterpress printing in the territory of present-day Switzerland."))
                .add_attribute(("start_date", "2008-06-01"))
                .add_attribute(("end_date", "2012-08-31"))
                .add_attribute((
                    "keywords",
                    Expression::from_iter([
                        (
                            "de",
                            Expression::from_iter([("Basel", Expression::from("Basel"))]),
                        ),
                        (
                            "en",
                            Expression::from_iter([
                                (
                                    "Contectualisation of images",
                                    Expression::from("Contectualisation of images"),
                                ),
                                ("Late Middle Ages", Expression::from("Late Middle Ages")),
                                (
                                    "Letterpress Printing",
                                    Expression::from("Letterpress Printing"),
                                ),
                            ]),
                        ),
                    ]),
                ))
                .build(),
        )
        .build();

    let input = r#"
          project "0803" {
            created_at  = "1637624150548721000"
            created_by  = "dsp-metadata-gui"
            shortcode   = "0803"
            teaser_text = "An art-scientific monograph of the richly illustrated early prints in Basel - the most important center of early letterpress printing in the territory of present-day Switzerland.",
            start_date  = "2008-06-01"
            end_date    = "2012-08-31",

            keywords = [
              {
                de = "Basel"
              },
              {
                en = "Contectualisation of images"
              },
              {
                en = "Late Middle Ages"
              },
              {
                en = "Letterpress Printing"
              }
            ]

            name "1" {
              de = "Die Bilderfolgen der Basler Frühdrucke: Spätmittelalterliche Didaxe als Bild-Text-Lektüre"
            }
            name "2" {
              en = "Incunabula"
            }
        
            description {
              de = "Eine kunstwissenschaftliche Monographie der reich bebilderten Frühdrucke in Basel - dem wichtigsten Zentrum des frühen Buchdrucks auf dem Gebiet der heutigen Schweiz - wird im vorliegenden Projekt erstmals seit über einem Jahrhundert ins Auge gefasst. Im Zentrum stehen 18 Werke aus vier verschiedenen Offizinen, welche insgesamt über 1000 Holzschnitte enthalten, die bedeutendsten überlieferten Basler Bilderfolgen des Spätmittelalters nach den massiven Zerstörungen im Zuge der Reformation. Bei den Texten handelt es sich fast ausschliesslich um deutsche und lateinische Kompilationen religiösen, didaktischen Inhalts, darunter viele zeitgenössische und in Basel entstandene, neben Übersetzungen des 15. Jahrhunderts und vollständig überarbeiteten Ausgaben bereits verbreiteter Werke. Äusserst erfolgreiche Bücher wie das Narrenschiff oder der Heilsspiegel stehen neben kaum bekannten wie den seelsorgerischen Schriften des Kartäusers Ludwig Moser und des Franziskaners Johannes Meder. Die Analyse eines umfassenden Corpus bebilderter Frühdrucke fehlt in der neueren Forschung, welche sich bezüglich der Basler Produktion vorwiegend der Untersuchung der Produzentenkreise gewidmet, die Bilder dagegen - mit Ausnahme des Narrenschiffs - wenig beachtet hat. Sehr heterogen ist auch die Erforschung der Texte, von denen ein grosser Teil unediert geblieben ist, von anderen wiederum existieren ausführlich kommentierte Faksimileausgaben. Die bisherige Bild-Text-Forschung hat sich auf das Narrenschiff fokussiert.Neben der Quellenanalyse der Bilder und Texte strebt das Projekt eine umfassende Untersuchung der Bild-Text-Bezüge unter Berücksichtigung rezeptionsästethischer Fragestellungen an. Gefragt wird nach der Funktion der Bilder für die spätmittelalterlichen visuellen und auditiven Rezipienten. Dabei wird davon ausgegangen, dass es sich bei unseren Frühdrucken ausnahmslos um kalkulierte Bild-Text-Kompilationen handelt, welche einen reflektierten und kreativen Umgang ihrer Produzenten mit den drucktechnischen Möglichkeiten des neuen Mediums voraussetzen, wie z.B. mit der Möglichkeit der vielfältigen Kontextualisierung von Bildern. Die Analyse der Bild- und Textquellen liefert eine umfassende Fallstudie zum Medienwechsel zwischen Handschrift und Frühdruck, diejenige der Bild-Text-Bezüge im Spannungsfeld zwischen mündlicher Tradierung und schriftlicher Fixierung religiöser Didaxe eine Fallstudie zur spätmittelalterlichen Rezeptionsforschung.Methodisch knüpft das Projekt an rezeptionsästhetische Konzepte und Studien der jüngeren literaturwissenschaftlichen Forschung an, welche mit ikonographischen Analysen kombiniert werden. Durch diese Erweiterung textzentrierter methodischer Ansätze treten die Texte in Bezug auf die Bilder nicht mehr als übergeordnete, unabhängige Einheiten in den Blick, sondern die Konstruktion des Werksinnes erscheint im Wechselspiel der Medien. Der traditionelle Begriff der Buchillustration wird dadurch grundlegend revidiert.Neben mehreren Aufsätzen und einer Tagung sind eine Monographie in Buchform geplant, welche sich an die Fachwelt sowie an eine interessierte Öffentlichkeit wendet, ausserdem eine Internet-Publikation der Bild- und Textquellen im Rahmen der Zusammenarbeit mit der Basler Univeristätsbibliothek."
            }
        
            discipline {
              text {
                en = "10404 Visual arts and Art history"
              }
            }

            publication "1" {
              text = "Graf Kathrin (2011), Klerikersatiren in Bild und Text. Zur Kompilationstechnik im Narrenschiff, in Krause Karin u. Schellewald Barbara (ed.), 205-227."
            }
            publication "2" {
              text = "Krause Karin u. Schellewald Barbara (ed.) (2011), Bild und Text im Mittelalter."
            }
        
            url "1" "https://admin.dasch.swiss/project/3ABR_2i8QYGSIDvmP9mlEw" {
              text = "Discover Project Data"
            }
        
            dataset "1" {
              ref = "dsp-0803-dataset-000",
            }
        
            funder {
              ref = "dsp:snf"
            }
        
            grant {
              ref = "0803:snf-120378"
            }
          }
    "#;
    let project_metadata_from_hcl: Body = hcl::from_str(input).unwrap();

    assert_eq!(expected, project_metadata_from_hcl);
}
