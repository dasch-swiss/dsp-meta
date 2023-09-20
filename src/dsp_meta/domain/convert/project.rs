use hcl::Expression;
use tracing::warn;

use crate::domain::value::alternative_name::AlternativeName;
use crate::domain::value::description::Description;
use crate::domain::value::discipline::Discipline;
use crate::domain::value::keyword::Keyword;
use crate::domain::value::publication::Publication;
use crate::domain::value::spatial_coverage::SpacialCoverage;
use crate::domain::value::temporal_coverage::TemporalCoverage;
use crate::domain::value::url::Url;
use crate::domain::value::{
    ContactPoint, CreatedAt, CreatedBy, EndDate, HowToCite, Name, Shortcode, StartDate, TeaserText,
};
use crate::errors::DspMetaError;

pub struct ExtractedProjectAttributes {
    pub created_at: Option<CreatedAt>,
    pub created_by: Option<CreatedBy>,
    pub shortcode: Option<Shortcode>,
    pub name: Option<Name>,
    pub teaser_text: Option<TeaserText>,
    pub how_to_cite: Option<HowToCite>,
    pub start_date: Option<StartDate>,
    pub end_date: Option<EndDate>,
    pub contact_point: Option<ContactPoint>,
}

impl TryFrom<Vec<&hcl::Attribute>> for ExtractedProjectAttributes {
    type Error = DspMetaError;

    fn try_from(attributes: Vec<&hcl::Attribute>) -> Result<Self, Self::Error> {
        let mut created_at: Option<CreatedAt> = None;
        let mut created_by: Option<CreatedBy> = None;
        let mut shortcode: Option<Shortcode> = None;
        let mut name: Option<Name> = None;
        let mut teaser_text: Option<TeaserText> = None;
        let mut how_to_cite: Option<HowToCite> = None;
        let mut start_date: Option<StartDate> = None;
        let mut end_date: Option<EndDate> = None;
        let mut contact_point: Option<ContactPoint> = None;

        // FIXME: throw error on duplicate attributes
        for attribute in attributes {
            match attribute.key() {
                "created_at" => {
                    created_at = match attribute.expr() {
                        Expression::Number(value) => Ok(Some(CreatedAt(value.as_u64().unwrap()))), /* FIXME: get rid of unwrap */
                        _ => Err(DspMetaError::ParseProject(
                            "Parse error: created_at needs to be a number.".to_string(),
                        )),
                    }?
                }
                "created_by" => {
                    created_by = match attribute.expr() {
                        Expression::String(value) => Ok(Some(CreatedBy(value.to_owned()))),
                        _ => Err(DspMetaError::ParseProject(
                            "Parse error: created_by needs to be a string.".to_string(),
                        )),
                    }?
                }
                "shortcode" => {
                    shortcode = match attribute.expr() {
                        Expression::String(value) => Ok(Some(Shortcode(value.to_owned()))),
                        _ => Err(DspMetaError::ParseProject(
                            "Parse error: shortcode needs to be a string.".to_string(),
                        )),
                    }?;
                }
                "name" => {
                    name = match attribute.expr() {
                        Expression::String(value) => Ok(Some(Name(value.to_owned()))),
                        _ => Err(DspMetaError::ParseProject(
                            "Parse error: name needs to be a string.".to_string(),
                        )),
                    }?;
                }
                "teaser_text" => {
                    teaser_text = match attribute.expr() {
                        Expression::String(value) => Ok(Some(TeaserText(value.to_owned()))),
                        _ => Err(DspMetaError::ParseProject(
                            "Parse error: teaser_text needs to be a string.".to_string(),
                        )),
                    }?;
                }
                "how_to_cite" => {
                    how_to_cite = match attribute.expr() {
                        Expression::String(value) => Ok(Some(HowToCite(value.to_owned()))),
                        _ => Err(DspMetaError::ParseProject(
                            "Parse error: how_to_cite needs to be a string.".to_string(),
                        )),
                    }?;
                }
                "start_date" => {
                    start_date = match attribute.expr() {
                        Expression::String(value) => Ok(Some(StartDate(value.to_owned()))),
                        _ => Err(DspMetaError::ParseProject(
                            "Parse error: start_date needs to be a string.".to_string(),
                        )),
                    }?;
                }
                "end_date" => {
                    end_date = match attribute.expr() {
                        Expression::String(value) => Ok(Some(EndDate(value.to_owned()))),
                        _ => Err(DspMetaError::ParseProject(
                            "Parse error: end_date needs to be a string.".to_string(),
                        )),
                    }?;
                }
                "contact_point" => {
                    contact_point = match attribute.expr() {
                        Expression::String(value) => Ok(Some(ContactPoint(value.to_owned()))),
                        _ => Err(DspMetaError::ParseProject(
                            "Parse error: contact_point needs to be a string.".to_string(),
                        )),
                    }?;
                }
                _ => {
                    warn!("Parse error: unknown attribute '{}'.", attribute.key());
                }
            }
        }
        Ok(ExtractedProjectAttributes {
            created_at,
            created_by,
            shortcode,
            name,
            teaser_text,
            how_to_cite,
            start_date,
            end_date,
            contact_point,
        })
    }
}

const ALTERNATIVE_NAME_BLOCK: &str = "alternative_name";
const DESCRIPTION_BLOCK: &str = "description";
const URL_BLOCK: &str = "url";
const KEYWORD_BLOCK: &str = "keyword";
const DISCIPLINE_BLOCK: &str = "discipline";
const SPACIAL_COVERAGE_BLOCK: &str = "spacial_coverage";
const TEMPORAL_COVERAGE_BLOCK: &str = "temporal_coverage";
const PUBLICATION_BLOCK: &str = "publication";

#[derive(Debug, Default, PartialEq)]
pub struct ExtractedProjectBlocks {
    pub alternative_names: Vec<AlternativeName>,
    pub description: Option<Description>,
    pub url: Option<Url>,
    pub keywords: Vec<Keyword>,
    pub disciplines: Vec<Discipline>,
    pub spacial_coverages: Vec<SpacialCoverage>,
    pub temporal_coverages: Vec<TemporalCoverage>,
    pub publications: Vec<Publication>,
}

impl TryFrom<Vec<&hcl::Block>> for ExtractedProjectBlocks {
    type Error = DspMetaError;

    fn try_from(blocks: Vec<&hcl::Block>) -> Result<Self, Self::Error> {
        let mut alternative_names: Vec<AlternativeName> = vec![];
        let mut description: Option<Description> = None;
        let mut url: Option<Url> = None;
        let mut keywords: Vec<Keyword> = vec![];
        let mut disciplines: Vec<Discipline> = vec![];
        let mut spacial_coverages: Vec<SpacialCoverage> = vec![];
        let mut temporal_coverages: Vec<TemporalCoverage> = vec![];
        let mut publications: Vec<Publication> = vec![];

        for block in blocks {
            match block.identifier.as_str() {
                ALTERNATIVE_NAME_BLOCK => {
                    alternative_names.push(AlternativeName::try_from(block)?);
                }
                DESCRIPTION_BLOCK => {
                    if description.is_some() {
                        return Err(DspMetaError::ParseProject(
                            "Only one 'description' block allowed.".to_string(),
                        ));
                    }
                    description = Some(Description::try_from(block)?)
                }
                URL_BLOCK => {
                    if url.is_some() {
                        return Err(DspMetaError::ParseProject(
                            "Only one 'url' block allowed.".to_string(),
                        ));
                    }
                    url = Some(Url::try_from(block)?)
                }
                KEYWORD_BLOCK => keywords.push(Keyword::try_from(block)?),
                DISCIPLINE_BLOCK => disciplines.push(Discipline::try_from(block)?),
                SPACIAL_COVERAGE_BLOCK => spacial_coverages.push(SpacialCoverage::try_from(block)?),
                TEMPORAL_COVERAGE_BLOCK => {
                    temporal_coverages.push(TemporalCoverage::try_from(block)?)
                }
                PUBLICATION_BLOCK => publications.push(Publication::try_from(block)?),
                _ => {
                    // catch all
                    warn!("Parse error: unknown block '{}'.", block.identifier);
                }
            }
        }
        Ok(ExtractedProjectBlocks {
            alternative_names,
            description,
            url,
            keywords,
            disciplines,
            spacial_coverages,
            temporal_coverages,
            publications,
        })
    }
}

#[cfg(test)]
mod tests {
    use hcl::{block, Identifier, Number};
    use tracing_test::traced_test;

    use super::*;

    #[test]
    fn extract_created_at() {
        let attribute = hcl::Attribute::new("created_at", Number::from(1u64));
        let attributes = vec![&attribute];
        let result = ExtractedProjectAttributes::try_from(attributes).unwrap();
        assert_eq!(result.created_at.unwrap(), CreatedAt(1));
    }

    #[test]
    fn extract_created_by() {
        let attribute = hcl::Attribute::new("created_by", "someone");
        let attributes = vec![&attribute];
        let result = ExtractedProjectAttributes::try_from(attributes).unwrap();
        assert_eq!(result.created_by.unwrap(), CreatedBy("someone".to_owned()));
    }

    #[traced_test]
    #[test]
    fn warn_on_unknown_attribute() {
        let attribute = hcl::Attribute::new("gugus", "something");
        let attributes = vec![&attribute];
        let _ = ExtractedProjectAttributes::try_from(attributes);

        assert!(logs_contain("Parse error: unknown attribute 'gugus'"));
    }

    #[test]
    fn extract_alternative_names() {
        let input1 = block!(
            alternative_name {
                de = "name1_de"
                en = "name1_en"
                fr = "name1_fr"
            }
        );
        let input2 = block!(
            alternative_name {
                de = "name2_de"
                en = "name2_en"
                fr = "name2_fr"
            }
        );
        let blocks = vec![&input1, &input2];
        let result = ExtractedProjectBlocks::try_from(blocks).unwrap();
        assert_eq!(result.alternative_names.len(), 2);
    }

    #[test]
    fn extract_description() {
        let input = block!(
            description {
                de = "descr_de"
                en = "descr_en"
                fr = "descr_fr"
            }
        );
        let blocks = vec![&input];
        let result = ExtractedProjectBlocks::try_from(blocks).unwrap();
        assert!(result.description.is_some());
    }

    #[test]
    fn error_on_multiple_description_blocks() {
        let input1 = block!(
            description {
                de = "descr_de"
                en = "descr_en"
                fr = "descr_fr"
            }
        );
        let input2 = block!(
            description {
                de = "descr_de"
                en = "descr_en"
                fr = "descr_fr"
            }
        );
        let blocks = vec![&input1, &input2];
        let result = ExtractedProjectBlocks::try_from(blocks);
        assert!(result.is_err());
    }

    #[test]
    fn extract_single_url() {
        let input = block!(
            url {
                href = "https://data.dasch.swiss/dokubib/"
                label = "Project Website"
            }
        );
        let blocks = vec![&input];
        let result = ExtractedProjectBlocks::try_from(blocks).unwrap();
        assert!(result.url.is_some());
    }

    #[test]
    fn error_on_multiple_url_blocks() {
        let input1 = block!(
            url {
                href = "https://data.dasch.swiss/dokubib/"
                label = "Project Website"
            }
        );
        let input2 = block!(
            url {
                href = "https://data.dasch.swiss/dokubib/"
                label = "Project Website"
            }
        );
        let blocks = vec![&input1, &input2];
        let result = ExtractedProjectBlocks::try_from(blocks);
        assert!(result.is_err());
    }

    #[test]
    fn extract_keywords() {
        let input1 = block!(
            keyword {
                de = "keyword1_de"
                en = "keyword1_en"
                fr = "keyword1_fr"
            }

        );
        let input2 = block!(
            keyword {
                de = "keyword2_de"
                en = "keyword2_en"
                fr = "keyword2_fr"
            }
        );
        let blocks = vec![&input1, &input2];
        let result = ExtractedProjectBlocks::try_from(blocks).unwrap();
        assert_eq!(result.keywords.len(), 2);
    }

    #[test]
    fn extract_disciplines() {
        let input1 = block!(
            discipline skos {
                ref_id = "https://skos.um.es/unesco6/5501"
                description = "Local history"
                url = "https://skos.um.es/unesco6/5501"
            }
        );
        let blocks = vec![&input1];
        let result = ExtractedProjectBlocks::try_from(blocks).unwrap();
        assert_eq!(result.disciplines.len(), 1);
    }

    #[test]
    fn extract_spacial_coverages() {
        let input1 = block!(
            spacial_coverage geonames {
                ref_id = "1234"
                description = "A description"
                url = "https://geonames.org/1234"
            }
        );
        let blocks = vec![&input1];
        let result = ExtractedProjectBlocks::try_from(blocks).unwrap();
        assert_eq!(result.spacial_coverages.len(), 1);
    }

    #[test]
    fn extract_temporal_coverage() {
        let input1 = block!(
            temporal_coverage chronontology {
                ref_id = "https://chronontology.dainst.org/period/INtagfT8h7Fs"
                description = "20th and 21st Centuries"
                url = "https://chronontology.dainst.org/period/INtagfT8h7Fs"
            }
        );
        let blocks = vec![&input1];
        let result = ExtractedProjectBlocks::try_from(blocks).unwrap();
        assert_eq!(result.temporal_coverages.len(), 1);
    }

    #[test]
    fn extract_publications() {
        let input1 = block!(
            publication {
                text = "A publication"
            }
        );
        let blocks = vec![&input1];
        let result = ExtractedProjectBlocks::try_from(blocks).unwrap();
        assert_eq!(result.publications.len(), 1);
    }

    #[traced_test]
    #[test]
    fn warn_on_unknown_block() {
        let block = hcl::Block::new(Identifier::new("gugus").unwrap());
        let blocks = vec![&block];
        let _ = ExtractedProjectBlocks::try_from(blocks);

        assert!(logs_contain("Parse error: unknown block 'gugus'"));
    }
}
