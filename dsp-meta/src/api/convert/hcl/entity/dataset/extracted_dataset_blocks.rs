use dsp_domain::metadata::value::attribution::Attribution;
use dsp_domain::metadata::value::language::Language;
use dsp_domain::metadata::value::license::License;
use dsp_domain::metadata::value::r#abstract::Abstract;
use dsp_domain::metadata::value::url::Url;
use tracing::warn;

use crate::api::convert::hcl::hcl_block::HclBlock;
use crate::error::DspMetaError;

const ABSTRACT_BLOCK: &str = "abstract";
const LICENSE_BLOCK: &str = "license";
const LANGUAGE_BLOCK: &str = "language";
const ATTRIBUTION_BLOCK: &str = "attribution";
const ALTERNATIVE_TITLE_BLOCK: &str = "alternative_title";
const URL_BLOCK: &str = "url";

#[derive(Debug, Default, PartialEq)]
pub struct ExtractedDatasetBlocks {
    pub abstracts: Vec<Abstract>,
    pub licenses: Vec<License>,
    pub languages: Vec<Language>,
    pub attributions: Vec<Attribution>,
    pub urls: Vec<Url>,
}

impl TryFrom<Vec<&hcl::Block>> for ExtractedDatasetBlocks {
    type Error = DspMetaError;

    fn try_from(blocks: Vec<&hcl::Block>) -> Result<Self, Self::Error> {
        let mut abstracts: Vec<Abstract> = vec![];
        let mut licenses: Vec<License> = vec![];
        let mut languages: Vec<Language> = vec![];
        let mut attributions: Vec<Attribution> = vec![];
        let mut urls: Vec<Url> = vec![];

        for block in blocks {
            match block.identifier.as_str() {
                // ABSTRACT_BLOCK => abstracts.push(HclBlock(block).try_into()?),
                // LICENSE_BLOCK => licenses.push(HclBlock(block).try_into()?),
                // LANGUAGE_BLOCK => languages.push(HclBlock(block).try_into()?),
                // ATTRIBUTION_BLOCK => attributions.push(HclBlock(block).try_into()?),
                // URL_BLOCK => urls.push(HclBlock(block).try_into()?),
                _ => {
                    // catch all
                    warn!(
                        "Parse error: unknown or not implemented block '{}'.",
                        block.identifier
                    );
                }
            }
        }
        Ok(ExtractedDatasetBlocks {
            abstracts,
            licenses,
            languages,
            attributions,
            urls,
        })
    }
}

#[cfg(test)]
mod tests {
    use hcl::{block, Identifier};
    use tracing_test::traced_test;

    use crate::api::convert::hcl::entity::dataset::extracted_dataset_blocks::ExtractedDatasetBlocks;

    #[test]
    fn extract_abstracts() {
        let input1 = block!(
            abstract {
                de = "abstract1_de"
                en = "abstract1_en"
                fr = "abstract1_fr"
            }
        );
        let input2 = block!(
            abstract {
                de = "abstract2_de"
                en = "abstract2_en"
                fr = "abstract2_fr"
            }
        );
        let blocks = vec![&input1, &input2];
        let result = ExtractedDatasetBlocks::try_from(blocks).unwrap();
        assert_eq!(result.abstracts.len(), 2);
    }

    #[traced_test]
    #[test]
    fn warn_on_unknown_block() {
        let block = hcl::Block::new(Identifier::new("gugus").unwrap());
        let blocks = vec![&block];
        let _ = ExtractedDatasetBlocks::try_from(blocks);

        assert!(logs_contain(
            " Parse error: unknown or not implemented block 'gugus'"
        ));
    }
}
