use std::collections::HashMap;

use hcl::{Attribute, Expression};
use tracing::warn;

use crate::domain::{
    CreatedAt, CreatedBy, EndDate, HowToCite, Name, ProjectValue, Shortcode, StartDate, TeaserText,
};
use crate::errors::DspMetaError;

pub struct ExtractedAttributes {
    pub created_at: Option<CreatedAt>,
    pub created_by: Option<CreatedBy>,
    pub shortcode: Option<Shortcode>,
    pub name: Option<Name>,
    pub teaser_text: Option<TeaserText>,
    pub how_to_cite: Option<HowToCite>,
    pub start_date: Option<StartDate>,
    pub end_date: Option<EndDate>,
}

pub fn extract_project_attributes(
    attributes: Vec<&Attribute>,
) -> Result<ExtractedAttributes, DspMetaError> {
    let mut results: HashMap<&str, ProjectValue> = HashMap::new();

    let mut created_at: Option<CreatedAt> = None;
    let mut created_by: Option<CreatedBy> = None;
    let mut shortcode: Option<Shortcode> = None;
    let mut name: Option<Name> = None;
    let mut teaser_text: Option<TeaserText> = None;
    let mut how_to_cite: Option<HowToCite> = None;
    let mut start_date: Option<StartDate> = None;
    let mut end_date: Option<EndDate> = None;

    for attribute in attributes {
        match attribute.key() {
            "created_at" => {
                created_at = match attribute.expr() {
                    Expression::Number(value) => Ok(Some(CreatedAt(value.as_u64().unwrap()))), /* FIXME: unwrap */
                    _ => Err(DspMetaError::ParseProject(
                        "Parse error: created_at needs to be a number.",
                    )),
                }?
            }
            "created_by" => {
                created_by = match attribute.expr() {
                    Expression::String(value) => Ok(Some(CreatedBy(value.to_string()))),
                    _ => Err(DspMetaError::ParseProject(
                        "Parse error: created_by needs to be a string.",
                    )),
                }?
            }
            "shortcode" => {
                let shortcode = match attribute.expr() {
                    Expression::String(value) => Ok(Some(Shortcode(value.to_string()))),
                    _ => Err(DspMetaError::ParseProject(
                        "Parse error: shortcode needs to be a string.",
                    )),
                }?;
                results.insert("shortcode", shortcode);
            }
            "name" => {
                let name = match attribute.expr() {
                    Expression::String(value) => Ok(Some(Name(value.to_string()))),
                    _ => Err(DspMetaError::ParseProject(
                        "Parse error: name needs to be a string.",
                    )),
                }?;
                results.insert("name", name);
            }
            "teaser_text" => {
                let teaser_text = match attribute.expr() {
                    Expression::String(value) => Ok(Some(TeaserText(value.to_string()))),
                    _ => Err(DspMetaError::ParseProject(
                        "Parse error: teaser_text needs to be a string.",
                    )),
                }?;
                results.insert("teaser_text", teaser_text);
            }
            "how_to_cite" => {
                let how_to_cite = match attribute.expr() {
                    Expression::String(value) => Ok(ProjectValue::HowToCite(HowToCite::new(value))),
                    _ => Err(DspMetaError::ParseProject(
                        "Parse error: how_to_cite needs to be a string.",
                    )),
                }?;
                results.insert("how_to_cite", how_to_cite);
            }
            "start_date" => {
                let start_date = match attribute.expr() {
                    Expression::String(value) => Ok(ProjectValue::StartDate(StartDate::new(value))),
                    _ => Err(DspMetaError::ParseProject(
                        "Parse error: start_date needs to be a string.",
                    )),
                }?;
                results.insert("start_date", start_date);
            }
            "end_date" => {
                let end_date = match attribute.expr() {
                    Expression::String(value) => Ok(ProjectValue::EndDate(EndDate::new(value))),
                    _ => Err(DspMetaError::ParseProject(
                        "Parse error: end_date needs to be a string.",
                    )),
                }?;
                results.insert("end_date", end_date);
            }
            _ => {
                warn!("Parse error: unknown attribute '{}'.", attribute.key());
            }
        }
    }
    Ok(ExtractedAttributes {
        created_at,
        created_by,
        shortcode,
        name,
        teaser_text,
        how_to_cite,
        start_date,
        end_date,
    })
}

#[cfg(test)]
mod tests {
    use hcl::Number;
    use tracing_test::traced_test;

    use super::*;

    #[test]
    fn extract_created_at() {
        let attribute = Attribute::new("created_at", Number::from(1u64));
        let attributes = vec![&attribute];
        let result = extract_project_attributes(attributes).unwrap();
        assert_eq!(result.created_at.unwrap(), CreatedAt(1));
    }

    fn extract_created_by() {
        let attribute = Attribute::new("created_by", "someone");
        let attributes = vec![&attribute];
        let result = extract_project_attributes(attributes).unwrap();
        assert_eq!(result.created_by.unwrap(), CreatedBy::from("someone"));
    }

    #[traced_test]
    #[test]
    fn warn_on_unknown_attribute() {
        let attribute = Attribute::new("gugus", "something");
        let attributes = vec![&attribute];
        let _ = extract_project_attributes(attributes);

        assert!(logs_contain("Parse error: unknown attribute 'gugus'"));
    }
}
