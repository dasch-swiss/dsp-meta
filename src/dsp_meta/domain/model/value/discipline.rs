use serde::Serialize;

use crate::domain::model::value::lang_text_data::LangTextData;
use crate::domain::model::value::ref_data::RefData;
use crate::error::DspMetaError;

/// The discipline of a project can be defined in two ways:
/// 1. A reference to a discipline defined in an external reference system (e.g. SNF or SKOS)
/// 2. A text description of the discipline
///
/// Example:
/// ```hcl
/// discipline skos {
///     ref_id = "https://skos.um.es/unesco6/5501"
///     description = "Local history"
///     url = "https://skos.um.es/unesco6/5501"
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum Discipline {
    Skos(RefData),
    Snf(RefData),
    Text(LangTextData),
}

impl TryFrom<&hcl::Block> for Discipline {
    type Error = DspMetaError;

    fn try_from(block: &hcl::Block) -> Result<Self, Self::Error> {
        if block.identifier.as_str() != "discipline" {
            let msg = format!(
                "The passed block is not named correctly. Expected 'discipline', however got '{}' instead.",
                block.identifier.as_str()
            );
            return Err(DspMetaError::CreateValueObject(msg));
        }

        if block.labels.len() != 1 {
            return Err(DspMetaError::CreateValueObject("The passed number of block labels is not correct. Expected '1', namely 'reference data type' (e.g., 'skos').".to_string()));
        }

        let reference_data_type = block.labels.first().ok_or_else(|| {
            DspMetaError::CreateValueObject(
                "The passed discipline block is missing the reference data type label.".to_string(),
            )
        })?;

        let attributes: Vec<&hcl::Attribute> = block.body.attributes().collect();

        match reference_data_type.as_str() {
            "skos" => {
                let ref_data = RefData::try_from(attributes)?;
                Ok(Discipline::Skos(ref_data))
            }
            "snf" => {
                let ref_data = RefData::try_from(attributes)?;
                Ok(Discipline::Snf(ref_data))
            }
            "text" => {
                let text_data = LangTextData::try_from(attributes)?;
                Ok(Discipline::Text(text_data))
            }
            _ => {
                Err(DspMetaError::CreateValueObject("The passed discipline block is missing the correct reference data type label: 'skos', 'snf', or 'text'.".to_string()))
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use tracing_test::traced_test;

    use super::*;
    use crate::domain::model::value::iso_code::IsoCode;

    #[test]
    #[traced_test]
    fn test_try_from_block_with_skos() {
        let block = hcl::block!(
            discipline skos {
                    ref_id = "https://skos.um.es/unesco6/5501"
                    description = "Local history"
                    url = "https://skos.um.es/unesco6/5501"
            }
        );

        let input = Discipline::try_from(&block).unwrap();
        let expected = Discipline::Skos(RefData {
            ref_id: "https://skos.um.es/unesco6/5501".to_string(),
            description: "Local history".to_string(),
            url: url::Url::parse("https://skos.um.es/unesco6/5501").unwrap(),
        });

        assert_eq!(input, expected);
    }

    #[test]
    #[traced_test]
    fn test_try_from_block_with_text() {
        let block = hcl::block!(
            discipline text {
                    de = "Lokalgeschichte"
                    en = "Local history"
                    fr = "Histoire locale"
            }
        );

        let input = Discipline::try_from(&block).unwrap();
        let expected = Discipline::Text(LangTextData(
            vec![
                (IsoCode::DE, "Lokalgeschichte".to_string()),
                (IsoCode::EN, "Local history".to_string()),
                (IsoCode::FR, "Histoire locale".to_string()),
            ]
            .into_iter()
            .collect(),
        ));

        assert_eq!(input, expected);
    }
}
