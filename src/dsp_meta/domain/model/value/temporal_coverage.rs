use crate::domain::model::value::lang_text_data::LangTextData;
use crate::domain::model::value::ref_data::RefData;
use crate::error::DspMetaError;

const TEMPORAL_COVERAGE: &str = "temporal_coverage";
const CHRONONTOLOGY: &str = "chronontology";
const PERIODO: &str = "periodo";

const TEXT: &str = "text";

#[derive(Debug, PartialEq)]
pub enum TemporalCoverage {
    Chronontology(RefData),
    Periodo(RefData),
    Text(LangTextData),
}

impl TryFrom<&hcl::Block> for TemporalCoverage {
    type Error = DspMetaError;

    fn try_from(block: &hcl::Block) -> Result<Self, Self::Error> {
        if block.identifier.as_str() != TEMPORAL_COVERAGE {
            let msg = format!(
                "The passed block is not named correctly. Expected 'temporal_coverage', however got '{}' instead.",
                block.identifier.as_str()
            );
            return Err(DspMetaError::CreateValueObject(msg));
        }

        if block.labels.len() != 1 {
            return Err(DspMetaError::CreateValueObject("The passed number of block labels is not correct. Expected '1', namely 'reference data type' (e.g., 'chronontology, periodo').".to_string()));
        }

        let reference_data_type = block.labels.first().ok_or_else(|| {
            DspMetaError::CreateValueObject(
                "The passed spacial_coverage block is missing the reference data type label."
                    .to_string(),
            )
        })?;

        let attributes: Vec<&hcl::Attribute> = block.body.attributes().collect();

        match reference_data_type.as_str() {
            CHRONONTOLOGY => {
                let ref_data = RefData::try_from(attributes)?;
                Ok(TemporalCoverage::Chronontology(ref_data))
            }
            PERIODO => {
                let ref_data = RefData::try_from(attributes)?;
                Ok(TemporalCoverage::Periodo(ref_data))
            }
            TEXT => {
                let text_data = LangTextData::try_from(attributes)?;
                Ok(TemporalCoverage::Text(text_data))
            }
            _ => {
                let msg = format!("The passed temporal_coverage block is missing the correct reference data type label. Expected one of '{}', '{}' or '{}'. Got '{}'.", CHRONONTOLOGY, PERIODO, TEXT, reference_data_type.as_str());
                Err(DspMetaError::CreateValueObject(msg))
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::model::value::ref_data::RefData;

    #[test]
    fn test_try_from_block_with_chonontology() {
        let block = hcl::block!(
            temporal_coverage chronontology {
                ref_id = "https://chronontology.dainst.org/period/INtagfT8h7Fs"
                description = "20th and 21st Centuries"
                url = "https://chronontology.dainst.org/period/INtagfT8h7Fs"
            }
        );

        let input = TemporalCoverage::try_from(&block).unwrap();
        let expected = TemporalCoverage::Chronontology(RefData {
            ref_id: "https://chronontology.dainst.org/period/INtagfT8h7Fs".to_string(),
            description: "20th and 21st Centuries".to_string(),
            url: "https://chronontology.dainst.org/period/INtagfT8h7Fs"
                .parse()
                .unwrap(),
        });

        assert_eq!(input, expected);
    }
}
