use std::collections::HashMap;

use hcl::{Attribute, Expression};
use tracing::warn;

use crate::domain::value_objects::{
    CreatedAt, CreatedBy, Datasets, EndDate, Funders, Grants, HowToCite, Name, ProjectValue,
    Shortcode, StartDate, TeaserText,
};
use crate::errors::DspMetaError;

pub fn parse_project_attributes(
    attributes: Vec<&Attribute>,
) -> Result<HashMap<&str, ProjectValue>, DspMetaError> {
    let mut results: HashMap<&str, ProjectValue> = HashMap::new();

    for attribute in attributes {
        match attribute.key() {
            "created_at" => {
                parse_created_at(&mut results, attribute)?;
            }
            "created_by" => {
                parse_created_by(&mut results, attribute)?;
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
            "datasets" => {
                let datasets = match attribute.expr() {
                    Expression::Array(value) => {
                        let mut datasets_value: Vec<String> = Vec::new();
                        for element in value {
                            match element {
                                Expression::String(value) => {
                                    datasets_value.push(value.to_string());
                                }
                                _ => {
                                    return Err(DspMetaError::ParseProject(
                                        "Parse error: datasets needs to be a list of strings.",
                                    ))
                                }
                            }
                        }
                        Ok(ProjectValue::Datasets(Datasets::new(datasets_value)))
                    }
                    _ => Err(DspMetaError::ParseProject(
                        "Parse error: datasets needs to be a list of strings.",
                    )),
                }?;
                results.insert("datasets", datasets);
            }
            "funders" => {
                let funders = match attribute.expr() {
                    Expression::Array(value) => {
                        let mut funders_value: Vec<String> = Vec::new();
                        for element in value {
                            match element {
                                Expression::String(value) => {
                                    funders_value.push(value.to_string());
                                }
                                _ => {
                                    return Err(DspMetaError::ParseProject(
                                        "Parse error: funders needs to be a list of strings.",
                                    ))
                                }
                            }
                        }
                        Ok(ProjectValue::Funders(Funders::new(funders_value)))
                    }
                    _ => Err(DspMetaError::ParseProject(
                        "Parse error: funders needs to be a list of strings.",
                    )),
                }?;
                results.insert("funders", funders);
            }
            "grants" => {
                let grants = match attribute.expr() {
                    Expression::Array(value) => {
                        let mut grants_value: Vec<String> = Vec::new();
                        for element in value {
                            match element {
                                Expression::String(value) => {
                                    grants_value.push(value.to_string());
                                }
                                _ => {
                                    return Err(DspMetaError::ParseProject(
                                        "Parse error: grants needs to be a list of strings.",
                                    ))
                                }
                            }
                        }
                        Ok(ProjectValue::Grants(Grants::new(grants_value)))
                    }
                    _ => Err(DspMetaError::ParseProject(
                        "Parse error: grants needs to be a list of strings.",
                    )),
                }?;
                results.insert("grants", grants);
            }
            _ => {
                warn!("Parse error: unknown attribute '{}'.", attribute.key());
            }
        }
    }
    Ok(results)
}

fn parse_created_by(
    results: &mut HashMap<&str, ProjectValue>,
    attribute: &Attribute,
) -> Result<(), DspMetaError> {
    let created_by = match attribute.expr() {
        Expression::String(value) => Ok(ProjectValue::CreatedBy(CreatedBy::new(value))),
        _ => Err(DspMetaError::ParseProject(
            "Parse error: created_by needs to be a string.",
        )),
    }?;
    results.insert("created_by", created_by);
    Ok(())
}

fn parse_created_at(
    results: &mut HashMap<&str, ProjectValue>,
    attribute: &Attribute,
) -> Result<(), DspMetaError> {
    let created_at = match attribute.expr() {
        Expression::Number(value) => Ok(ProjectValue::CreatedAt(CreatedAt::new(
            value.as_u64().unwrap(),
        ))),
        _ => Err(DspMetaError::ParseProject(
            "Parse error: created_at needs to be a number.",
        )),
    }?;
    results.insert("created_at", created_at);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn created_by() {
        let mut results: HashMap<&str, ProjectValue> = HashMap::new();
        let attribute = Attribute::new("created_by", Expression::String("John Doe".to_string()));
        parse_created_by(&mut results, &attribute).unwrap();
        assert_eq!(
            results.get("created_by"),
            Some(&ProjectValue::CreatedBy(CreatedBy::new("John Doe")))
        );
    }
}
