use std::cell::OnceCell;

use dsp_domain::metadata::value::identifier::DatasetId;
use dsp_domain::metadata::value::{CreatedAt, CreatedBy, Title};
use hcl::Expression;
use tracing::warn;

use crate::error::DspMetaError;

pub struct ExtractedDatasetAttributes {
    pub id: OnceCell<DatasetId>,
    pub created_at: OnceCell<CreatedAt>,
    pub created_by: OnceCell<CreatedBy>,
    pub title: OnceCell<Title>,
}

impl TryFrom<Vec<&hcl::Attribute>> for ExtractedDatasetAttributes {
    type Error = DspMetaError;

    fn try_from(attributes: Vec<&hcl::Attribute>) -> Result<Self, Self::Error> {
        let id: OnceCell<DatasetId> = OnceCell::new();
        let created_at: OnceCell<CreatedAt> = OnceCell::new();
        let created_by: OnceCell<CreatedBy> = OnceCell::new();
        let title: OnceCell<Title> = OnceCell::new();

        for attribute in attributes {
            match attribute.key() {
                "id" => {
                    let extracted_id = match attribute.expr() {
                        Expression::String(value) => Ok(DatasetId(value.to_owned())),
                        _ => Err(DspMetaError::ParseDataset(
                            "Parse error: id needs to be a string.".to_string(),
                        )),
                    }?;
                    id.set(extracted_id).map_err(|_| {
                        DspMetaError::ParseDataset(
                            "Parse error: id needs to be unique.".to_string(),
                        )
                    })?;
                }
                "created_at" => {
                    let extracted_created_at = match attribute.expr() {
                        Expression::Number(value) => Ok(CreatedAt(value.as_u64().unwrap())), /* FIXME: get rid of unwrap */
                        _ => Err(DspMetaError::ParseDataset(
                            "Parse error: created_at needs to be a number.".to_string(),
                        )),
                    }?;
                    created_at.set(extracted_created_at).map_err(|_| {
                        DspMetaError::ParseDataset(
                            "Parse error: created_at needs to be unique.".to_string(),
                        )
                    })?;
                }
                "created_by" => {
                    let extracted_created_by = match attribute.expr() {
                        Expression::String(value) => Ok(CreatedBy(value.to_owned())),
                        _ => Err(DspMetaError::ParseDataset(
                            "Parse error: created_by needs to be a string.".to_string(),
                        )),
                    }?;
                    created_by.set(extracted_created_by).map_err(|_| {
                        DspMetaError::ParseDataset(
                            "Parse error: created_by needs to be unique.".to_string(),
                        )
                    })?;
                }
                "title" => {
                    let extracted_title = match attribute.expr() {
                        Expression::String(value) => Ok(Title(value.to_owned())),
                        _ => Err(DspMetaError::ParseDataset(
                            "Parse error: title needs to be a string.".to_string(),
                        )),
                    }?;
                    title.set(extracted_title).map_err(|_| {
                        DspMetaError::ParseDataset(
                            "Parse error: title needs to be unique.".to_string(),
                        )
                    })?;
                }
                _ => {
                    warn!("Parse error: unknown attribute '{}'.", attribute.key());
                }
            }
        }
        Ok(ExtractedDatasetAttributes {
            id,
            created_at,
            created_by,
            title,
        })
    }
}

#[cfg(test)]
mod tests {
    use hcl::Number;
    use tracing_test::traced_test;

    use super::*;

    #[test]
    fn extract_created_at() {
        let attribute = hcl::Attribute::new("created_at", Number::from(1u64));
        let attributes = vec![&attribute];
        let mut result = ExtractedDatasetAttributes::try_from(attributes).unwrap();
        assert_eq!(result.created_at.take().unwrap(), CreatedAt(1));
    }

    #[test]
    fn extract_created_by() {
        let attribute = hcl::Attribute::new("created_by", "someone");
        let attributes = vec![&attribute];
        let mut result = ExtractedDatasetAttributes::try_from(attributes).unwrap();
        assert_eq!(
            result.created_by.take().unwrap(),
            CreatedBy("someone".to_string())
        );
    }

    #[traced_test]
    #[test]
    fn warn_on_unknown_attribute() {
        let attribute = hcl::Attribute::new("gugus", "something");
        let attributes = vec![&attribute];
        let _ = ExtractedDatasetAttributes::try_from(attributes);

        assert!(logs_contain("Parse error: unknown attribute 'gugus'"));
    }
}
