use crate::domain::value::ref_data::RefData;
use crate::errors::DspMetaError;

#[derive(Debug, PartialEq)]
pub enum SpacialCoverage {
    Geonames(RefData),
}

impl TryFrom<&hcl::Block> for SpacialCoverage {
    type Error = DspMetaError;

    fn try_from(block: &hcl::Block) -> Result<Self, Self::Error> {
        if block.identifier.as_str() != "spacial_coverage" {
            let msg = format!(
                "The passed block is not named correctly. Expected 'spacial_coverage', however got '{}' instead.",
                block.identifier.as_str()
            );
            return Err(DspMetaError::CreateValueObject(msg));
        }

        if block.labels.len() != 1 {
            return Err(DspMetaError::CreateValueObject("The passed number of block labels is not correct. Expected '1', namely 'reference data type' (e.g., 'geonames').".to_string()));
        }

        let reference_data_type = block.labels.first().ok_or_else(|| {
            DspMetaError::CreateValueObject(
                "The passed spacial_coverage block is missing the reference data type label."
                    .to_string(),
            )
        })?;

        let attributes: Vec<&hcl::Attribute> = block.body.attributes().collect();

        match reference_data_type.as_str() {
            "geonames" => {
                let ref_data = RefData::try_from(attributes)?;
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
    use crate::domain::value::ref_data::RefData;
    use crate::domain::value::spatial_coverage::SpacialCoverage;

    #[test]
    fn test_try_from_block_with_geonames() {
        let block = hcl::block!(
            spacial_coverage geonames {
                ref_id = "1234"
                description = "A description"
                url = "https://geonames.org/1234"
            }
        );

        let input = SpacialCoverage::try_from(&block).unwrap();
        let expected = SpacialCoverage::Geonames(RefData {
            ref_id: "1234".to_string(),
            description: "A description".to_string(),
            url: "https://geonames.org/1234".parse().unwrap(),
        });

        assert_eq!(input, expected);
    }
}
