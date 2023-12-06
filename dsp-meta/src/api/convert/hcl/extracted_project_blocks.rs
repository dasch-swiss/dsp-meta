use dsp_domain::metadata::value::alternative_name::AlternativeName;
use dsp_domain::metadata::value::description::Description;
use dsp_domain::metadata::value::discipline::Discipline;
use dsp_domain::metadata::value::keyword::Keyword;
use dsp_domain::metadata::value::publication::Publication;
use dsp_domain::metadata::value::spatial_coverage::SpacialCoverage;
use dsp_domain::metadata::value::temporal_coverage::TemporalCoverage;
use dsp_domain::metadata::value::url::Url;
use tracing::warn;

use crate::api::convert::hcl::hcl_block::HclBlock;
use crate::error::DspMetaError;

const ALTERNATIVE_NAME_BLOCK: &str = "alternative_name";
const DESCRIPTION_BLOCK: &str = "description";
const URL_BLOCK: &str = "url";
const KEYWORD_BLOCK: &str = "keyword";
const DISCIPLINE_BLOCK: &str = "discipline";
const SPACIAL_COVERAGE_BLOCK: &str = "spatial_coverage";
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
                ALTERNATIVE_NAME_BLOCK => alternative_names.push(HclBlock(block).try_into()?),
                DESCRIPTION_BLOCK => {
                    if description.is_some() {
                        return Err(DspMetaError::ParseProject(
                            "Only one 'description' block allowed.".to_string(),
                        ));
                    }
                    description = Some(HclBlock(block).try_into()?)
                }
                URL_BLOCK => {
                    if url.is_some() {
                        return Err(DspMetaError::ParseProject(
                            "Only one 'url' block allowed.".to_string(),
                        ));
                    }
                    url = Some(HclBlock(block).try_into()?)
                }
                KEYWORD_BLOCK => keywords.push(HclBlock(block).try_into()?),
                DISCIPLINE_BLOCK => disciplines.push(HclBlock(block).try_into()?),
                SPACIAL_COVERAGE_BLOCK => spacial_coverages.push(HclBlock(block).try_into()?),
                TEMPORAL_COVERAGE_BLOCK => temporal_coverages.push(HclBlock(block).try_into()?),
                PUBLICATION_BLOCK => publications.push(HclBlock(block).try_into()?),
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
    use hcl::{block, Identifier};
    use tracing_test::traced_test;

    use super::*;

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
