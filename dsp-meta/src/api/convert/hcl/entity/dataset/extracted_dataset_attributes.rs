use std::cell::OnceCell;

use dsp_domain::metadata::value::access::Access;
use dsp_domain::metadata::value::data_type::DataType;
use dsp_domain::metadata::value::identifier::DatasetId;
use dsp_domain::metadata::value::status::Status;
use dsp_domain::metadata::value::{CreatedAt, CreatedBy, DatePublished, HowToCite, Title};
use hcl::Expression;
use tracing::warn;

use crate::error::DspMetaError;

pub struct ExtractedDatasetAttributes {
    pub id: OnceCell<DatasetId>,                 // (1)
    pub created_at: OnceCell<CreatedAt>,         // (1)
    pub created_by: OnceCell<CreatedBy>,         // (1)
    pub title: OnceCell<Title>,                  // (1)
    pub status: OnceCell<Status>,                // (1)
    pub access_conditions: OnceCell<Access>,     // (1)
    pub how_to_cite: OnceCell<HowToCite>,        // (1)
    pub date_published: OnceCell<DatePublished>, // (0-1)
    pub type_of_data: Vec<DataType>,             // (1-n)
    pub alternative_titles: Vec<Title>,          // (0-n)
}

impl TryFrom<Vec<&hcl::Attribute>> for ExtractedDatasetAttributes {
    type Error = DspMetaError;

    fn try_from(attributes: Vec<&hcl::Attribute>) -> Result<Self, Self::Error> {
        let id: OnceCell<DatasetId> = OnceCell::new();
        let created_at: OnceCell<CreatedAt> = OnceCell::new();
        let created_by: OnceCell<CreatedBy> = OnceCell::new();
        let title: OnceCell<Title> = OnceCell::new();
        let status: OnceCell<Status> = OnceCell::new();
        let access_conditions: OnceCell<Access> = OnceCell::new();
        let how_to_cite: OnceCell<HowToCite> = OnceCell::new();
        let date_published: OnceCell<DatePublished> = OnceCell::new();
        let mut type_of_data: Vec<DataType> = vec![];
        let mut alternative_titles: Vec<Title> = vec![];

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
                "status" => {
                    let extracted_status = match attribute.expr() {
                        Expression::String(value) => Ok(Status::try_from(value.to_owned())?),
                        _ => Err(DspMetaError::ParseDataset(
                            "Parse error: status needs to be a string.".to_string(),
                        )),
                    }?;
                    status.set(extracted_status).map_err(|_| {
                        DspMetaError::ParseDataset(
                            "Parse error: status needs to be unique.".to_string(),
                        )
                    })?;
                }
                "access_conditions" => {
                    let extracted_access_conditions = match attribute.expr() {
                        Expression::String(value) => Ok(Access::try_from(value.to_owned())?),
                        _ => Err(DspMetaError::ParseDataset(
                            "Parse error: access_conditions needs to be a string.".to_string(),
                        )),
                    }?;
                    access_conditions
                        .set(extracted_access_conditions)
                        .map_err(|_| {
                            DspMetaError::ParseDataset(
                                "Parse error: access_conditions needs to be unique.".to_string(),
                            )
                        })?;
                }
                "how_to_cite" => {
                    let extracted_how_to_cite = match attribute.expr() {
                        Expression::String(value) => Ok(HowToCite(value.to_owned())),
                        _ => Err(DspMetaError::ParseDataset(
                            "Parse error: how_to_cite needs to be a string.".to_string(),
                        )),
                    }?;
                    how_to_cite.set(extracted_how_to_cite).map_err(|_| {
                        DspMetaError::ParseDataset(
                            "Parse error: how_to_cite needs to be unique.".to_string(),
                        )
                    })?;
                }
                "date_published" => {
                    let extracted_date_published = match attribute.expr() {
                        Expression::Number(value) => Ok(DatePublished(value.as_u64().unwrap())),
                        _ => Err(DspMetaError::ParseDataset(
                            "Parse error: date_published needs to be a string.".to_string(),
                        )),
                    }?;
                    date_published.set(extracted_date_published).map_err(|_| {
                        DspMetaError::ParseDataset(
                            "Parse error: date_published needs to be unique.".to_string(),
                        )
                    })?;
                }
                "type_of_data" => {
                    type_of_data = match attribute.expr() {
                        Expression::Array(values) => {
                            let mut data_types = vec![];
                            for value in values {
                                match value {
                                    Expression::String(value) => {
                                        data_types.push(DataType::try_from(value.to_owned())?)
                                    }
                                    _ => {
                                        return Err(DspMetaError::ParseDataset(
                                            "Parse error: type_of_data value needs to be a string."
                                                .to_string(),
                                        ))
                                    }
                                }
                            }
                            Ok(data_types)
                        }
                        _ => Err(DspMetaError::ParseDataset(
                            "Parse error: type_of_data needs to be a list of strings.".to_string(),
                        )),
                    }?;
                }
                "alternative_titles" => {
                    alternative_titles = match attribute.expr() {
                        Expression::Array(values) => {
                            let mut titles = vec![];
                            for value in values {
                                match value {
                                    Expression::String(value) => {
                                        titles.push(Title(value.to_owned()))
                                    }
                                    _ => {
                                        return Err(DspMetaError::ParseDataset(
                                            "Parse error: alternative_titles value needs to be a string."
                                                .to_string(),
                                        ))
                                    }
                                }
                            }
                            Ok(titles)
                        }
                        _ => Err(DspMetaError::ParseDataset(
                            "Parse error: alternative_titles needs to be a list of strings."
                                .to_string(),
                        )),
                    }?;
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
            status,
            access_conditions,
            how_to_cite,
            date_published,
            type_of_data,
            alternative_titles,
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
