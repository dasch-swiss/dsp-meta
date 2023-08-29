use serde::{Deserialize, Serialize};

use crate::domain::converter::project::convert_project_block;
use crate::domain::{
    AlternativeNames, CreatedAt, CreatedBy, Description, EndDate, HowToCite, Name, Shortcode,
    StartDate, TeaserText, ID,
};
use crate::errors::DspMetaError;

// no need for smart constructors here, as there is no validation happening
#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
pub struct Project {
    pub id: ID,
    pub created_at: CreatedAt,
    pub created_by: CreatedBy,
    pub shortcode: Shortcode,
    pub name: Name,
    pub alternative_names: AlternativeNames,
    pub teaser_text: TeaserText,
    pub description: Description,
    pub how_to_cite: HowToCite,
    pub start_date: StartDate,
    pub end_date: Option<EndDate>,
}

impl TryFrom<&hcl::Block> for Project {
    type Error = crate::errors::DspMetaError;

    fn try_from(project_block: &hcl::Block) -> Result<Self, Self::Error> {
        convert_project_block(project_block)
    }
}

/// Helper function that extracts the project from a list of projects.
/// Our constraint is that there must be exactly one project defined per metadata file.
pub fn extract_project(projects: Vec<Project>) -> Result<Project, DspMetaError> {
    if projects.len() > 1 {
        return Err(DspMetaError::ParseProject(
            "There can only be one project block.",
        ));
    }

    if projects.is_empty() {
        return Err(DspMetaError::ParseProject(
            "There needs to be a project block.",
        ));
    }

    Ok(projects[0].clone())
}

#[cfg(test)]
mod tests {
    use hcl::block;

    use super::*;
    use crate::domain::Shortcode;

    #[test]
    fn test_try_from_block() {
        let project_block = block!(
        project "0803" {
            created_at = 1637624150548721000u64
            created_by = "dsp-metadata-gui"
            shortcode  = "0803"
            name       = "Die Bilderfolgen der Basler Frühdrucke: Spätmittelalterliche Didaxe als Bild-Text-Lektüre"

            alternativeName "1" {
                en = "Incunabula"
            }

            teaser_text = "An art-scientific monograph of the richly illustrated early prints in Basel - the most important center of early letterpress printing in the territory of present-day Switzerland."

            description {
                de = "Eine kunstwissenschaftliche Monographie der reich bebilderten Frühdrucke in Basel - dem wichtigsten Zentrum des frühen Buchdrucks auf dem Gebiet der heutigen Schweiz - wird im vorliegenden Projekt erstmals seit über einem Jahrhundert ins Auge gefasst. Im Zentrum stehen 18 Werke aus vier verschiedenen Offizinen, welche insgesamt über 1000 Holzschnitte enthalten, die bedeutendsten überlieferten Basler Bilderfolgen des Spätmittelalters nach den massiven Zerstörungen im Zuge der Reformation. Bei den Texten handelt es sich fast ausschliesslich um deutsche und lateinische Kompilationen religiösen, didaktischen Inhalts, darunter viele zeitgenössische und in Basel entstandene, neben Übersetzungen des 15. Jahrhunderts und vollständig überarbeiteten Ausgaben bereits verbreiteter Werke. Äusserst erfolgreiche Bücher wie das Narrenschiff oder der Heilsspiegel stehen neben kaum bekannten wie den seelsorgerischen Schriften des Kartäusers Ludwig Moser und des Franziskaners Johannes Meder. Die Analyse eines umfassenden Corpus bebilderter Frühdrucke fehlt in der neueren Forschung, welche sich bezüglich der Basler Produktion vorwiegend der Untersuchung der Produzentenkreise gewidmet, die Bilder dagegen - mit Ausnahme des Narrenschiffs - wenig beachtet hat. Sehr heterogen ist auch die Erforschung der Texte, von denen ein grosser Teil unediert geblieben ist, von anderen wiederum existieren ausführlich kommentierte Faksimileausgaben. Die bisherige Bild-Text-Forschung hat sich auf das Narrenschiff fokussiert.Neben der Quellenanalyse der Bilder und Texte strebt das Projekt eine umfassende Untersuchung der Bild-Text-Bezüge unter Berücksichtigung rezeptionsästethischer Fragestellungen an. Gefragt wird nach der Funktion der Bilder für die spätmittelalterlichen visuellen und auditiven Rezipienten. Dabei wird davon ausgegangen, dass es sich bei unseren Frühdrucken ausnahmslos um kalkulierte Bild-Text-Kompilationen handelt, welche einen reflektierten und kreativen Umgang ihrer Produzenten mit den drucktechnischen Möglichkeiten des neuen Mediums voraussetzen, wie z.B. mit der Möglichkeit der vielfältigen Kontextualisierung von Bildern. Die Analyse der Bild- und Textquellen liefert eine umfassende Fallstudie zum Medienwechsel zwischen Handschrift und Frühdruck, diejenige der Bild-Text-Bezüge im Spannungsfeld zwischen mündlicher Tradierung und schriftlicher Fixierung religiöser Didaxe eine Fallstudie zur spätmittelalterlichen Rezeptionsforschung.Methodisch knüpft das Projekt an rezeptionsästhetische Konzepte und Studien der jüngeren literaturwissenschaftlichen Forschung an, welche mit ikonographischen Analysen kombiniert werden. Durch diese Erweiterung textzentrierter methodischer Ansätze treten die Texte in Bezug auf die Bilder nicht mehr als übergeordnete, unabhängige Einheiten in den Blick, sondern die Konstruktion des Werksinnes erscheint im Wechselspiel der Medien. Der traditionelle Begriff der Buchillustration wird dadurch grundlegend revidiert.Neben mehreren Aufsätzen und einer Tagung sind eine Monographie in Buchform geplant, welche sich an die Fachwelt sowie an eine interessierte Öffentlichkeit wendet, ausserdem eine Internet-Publikation der Bild- und Textquellen im Rahmen der Zusammenarbeit mit der Basler Univeristätsbibliothek."
            }

            url "https://admin.dasch.swiss/project/3ABR_2i8QYGSIDvmP9mlEw" {
                text = "Discover Project Data"
            }

            how_to_cite = "Incunabula"
            start_date = "2008-06-01"
            end_date   = "2012-08-31"

            keyword {
                de = "Basel"
            }

            discipline {
                text {
                    en = "10404 Visual arts and Art history"
                }
            }

            publication "1" {
                text = "Graf Kathrin (2011), Klerikersatiren in Bild und Text. Zur Kompilationstechnik im Narrenschiff, in Krause Karin u. Schellewald Barbara (ed.), 205-227."
            }
            }
        );
        let project = Project::try_from(&project_block).unwrap();
        assert_eq!(project.shortcode, Shortcode::new("0803"));
    }
}
