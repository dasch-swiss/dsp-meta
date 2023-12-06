use dsp_domain::metadata::value::ref_data::RefData;
use dsp_domain::metadata::value::spatial_coverage::SpacialCoverage;

use crate::api::convert::hcl::hcl_attribute::HclAttributes;
use crate::api::convert::hcl::hcl_block::HclBlock;
use crate::error::DspMetaError;

const SPATIAL_COVERAGE: &str = "spatial_coverage";
const GEONAMES: &str = "geonames";

impl<'a> TryInto<SpacialCoverage> for HclBlock<'a> {
    type Error = DspMetaError;

    fn try_into(self) -> Result<SpacialCoverage, Self::Error> {
        if self.0.identifier.as_str() != SPATIAL_COVERAGE {
            let msg = format!(
                "The passed block is not named correctly. Expected 'spacial_coverage', however got '{}' instead.",
                self.0.identifier.as_str()
            );
            return Err(DspMetaError::CreateValueObject(msg));
        }

        if self.0.labels.len() != 1 {
            return Err(DspMetaError::CreateValueObject("The passed number of block labels is not correct. Expected '1', namely 'reference data type' (e.g., 'geonames').".to_string()));
        }

        let reference_data_type = self.0.labels.first().ok_or_else(|| {
            DspMetaError::CreateValueObject(
                "The passed spacial_coverage block is missing the reference data type label."
                    .to_string(),
            )
        })?;

        let attributes: Vec<&hcl::Attribute> = self.0.body.attributes().collect();

        match reference_data_type.as_str() {
            GEONAMES => {
                let ref_data: RefData = HclAttributes(attributes).try_into()?;
                Ok(SpacialCoverage::Geonames(ref_data))
            }
            _ => {
                Err(DspMetaError::CreateValueObject("The passed spacial_coverage block is missing the correct reference data type label: 'geonames'.".to_string()))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_try_from_block_with_geonames() {
        let block = hcl::block!(
            spacial_coverage geonames {
                ref_id = "1234"
                description = "A description"
                url = "https://geonames.org/1234"
            }
        );

        let input: SpacialCoverage = HclBlock(&block).try_into().unwrap();
        let expected = SpacialCoverage::Geonames(RefData {
            ref_id: "1234".to_string(),
            description: "A description".to_string(),
            url: "https://geonames.org/1234".parse().unwrap(),
        });

        assert_eq!(input, expected);
    }
}
