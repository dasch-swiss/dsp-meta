use std::collections::HashMap;

use hcl::{Attribute, Expression};
use tracing::warn;

use crate::domain::{
    CreatedAt, CreatedBy, EndDate, HowToCite, Name, ProjectValue, Shortcode, StartDate, TeaserText,
};
use crate::errors::DspMetaError;

pub fn parse_project_attributes(
    attributes: Vec<&Attribute>,
) -> Result<HashMap<&str, ProjectValue>, DspMetaError> {
    let mut results: HashMap<&str, ProjectValue> = HashMap::new();

    for attribute in attributes {
        match attribute.key() {
            "created_at" => {
                results.insert(
                    "created_at",
                    ProjectValue::CreatedAt(parse_created_at(attribute)?),
                );
            }
            "created_by" => {
                results.insert(
                    "created_by",
                    ProjectValue::CreatedBy(parse_created_by(attribute)?),
                );
            }
            "shortcode" => {
                let shortcode = match attribute.expr() {
                    Expression::String(value) => Ok(ProjectValue::Shortcode(Shortcode::new(value))),
                    _ => Err(DspMetaError::ParseProject(
                        "Parse error: shortcode needs to be a string.",
                    )),
                }?;
                results.insert("shortcode", shortcode);
            }
            "name" => {
                let name = match attribute.expr() {
                    Expression::String(value) => Ok(ProjectValue::Name(Name::new(value))),
                    _ => Err(DspMetaError::ParseProject(
                        "Parse error: name needs to be a string.",
                    )),
                }?;
                results.insert("name", name);
            }
            "teaser_text" => {
                let teaser_text = match attribute.expr() {
                    Expression::String(value) => {
                        Ok(ProjectValue::TeaserText(TeaserText::new(value)))
                    }
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
    Ok(results)
}

fn parse_created_at(attribute: &Attribute) -> Result<CreatedAt, DspMetaError> {
    let created_at = match attribute.expr() {
        Expression::Number(value) => Ok(CreatedAt::new(value.as_u64().unwrap())),
        _ => Err(DspMetaError::ParseProject(
            "Parse error: created_at needs to be a number.",
        )),
    }?;
    Ok(created_at)
}

fn parse_created_by(attribute: &Attribute) -> Result<CreatedBy, DspMetaError> {
    let created_by = match attribute.expr() {
        Expression::String(value) => Ok(CreatedBy::new(value)),
        _ => Err(DspMetaError::ParseProject(
            "Parse error: created_by needs to be a string.",
        )),
    }?;
    Ok(created_by)
}

#[cfg(test)]
mod tests {
    use hcl::Number;

    use super::*;

    #[test]
    fn created_at() {
        let attribute = Attribute::new("created_at", Expression::Number(Number::from(123456789)));
        let created_at = parse_created_at(&attribute).unwrap();
        assert_eq!(created_at, CreatedAt::new(123456789));
    }

    #[test]
    fn created_by() {
        let attribute = Attribute::new("created_by", Expression::String("John Doe".to_string()));
        let created_by = parse_created_by(&attribute).unwrap();
        assert_eq!(created_by, CreatedBy::new("John Doe"));
    }
}
