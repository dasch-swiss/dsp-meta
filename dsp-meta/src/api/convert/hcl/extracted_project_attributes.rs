use dsp_domain::metadata::value::status::Status;
use dsp_domain::metadata::value::{
    ContactPoint, CreatedAt, CreatedBy, EndDate, HowToCite, Name, Shortcode, StartDate, TeaserText,
};
use hcl::Expression;
use tracing::warn;

use crate::error::DspMetaError;

pub struct ExtractedProjectAttributes {
    pub created_at: Option<CreatedAt>,
    pub created_by: Option<CreatedBy>,
    pub shortcode: Option<Shortcode>,
    pub name: Option<Name>,
    pub teaser_text: Option<TeaserText>,
    pub how_to_cite: Option<HowToCite>,
    pub start_date: Option<StartDate>,
    pub end_date: Option<EndDate>,
    pub status: Option<Status>,
    pub contact_point: Option<ContactPoint>,
}

impl TryFrom<Vec<&hcl::Attribute>> for ExtractedProjectAttributes {
    type Error = DspMetaError;

    fn try_from(attributes: Vec<&hcl::Attribute>) -> Result<Self, Self::Error> {
        let mut created_at: Option<CreatedAt> = None;
        let mut created_by: Option<CreatedBy> = None;
        let mut shortcode: Option<Shortcode> = None;
        let mut name: Option<Name> = None;
        let mut teaser_text: Option<TeaserText> = None;
        let mut how_to_cite: Option<HowToCite> = None;
        let mut start_date: Option<StartDate> = None;
        let mut end_date: Option<EndDate> = None;
        let mut status: Option<Status> = None;
        let mut contact_point: Option<ContactPoint> = None;

        // FIXME: throw error on duplicate attributes
        for attribute in attributes {
            match attribute.key() {
                "created_at" => {
                    created_at = match attribute.expr() {
                        Expression::Number(value) => Ok(Some(CreatedAt(value.as_u64().unwrap()))), /* FIXME: get rid of unwrap */
                        _ => Err(DspMetaError::ParseProject(
                            "Parse error: created_at needs to be a number.".to_string(),
                        )),
                    }?
                }
                "created_by" => {
                    created_by = match attribute.expr() {
                        Expression::String(value) => Ok(Some(CreatedBy(value.to_owned()))),
                        _ => Err(DspMetaError::ParseProject(
                            "Parse error: created_by needs to be a string.".to_string(),
                        )),
                    }?
                }
                "shortcode" => {
                    shortcode = match attribute.expr() {
                        Expression::String(value) => Ok(Some(Shortcode(value.to_owned()))),
                        _ => Err(DspMetaError::ParseProject(
                            "Parse error: shortcode needs to be a string.".to_string(),
                        )),
                    }?;
                }
                "name" => {
                    name = match attribute.expr() {
                        Expression::String(value) => Ok(Some(Name(value.to_owned()))),
                        _ => Err(DspMetaError::ParseProject(
                            "Parse error: name needs to be a string.".to_string(),
                        )),
                    }?;
                }
                "teaser_text" => {
                    teaser_text = match attribute.expr() {
                        Expression::String(value) => Ok(Some(TeaserText(value.to_owned()))),
                        _ => Err(DspMetaError::ParseProject(
                            "Parse error: teaser_text needs to be a string.".to_string(),
                        )),
                    }?;
                }
                "how_to_cite" => {
                    how_to_cite = match attribute.expr() {
                        Expression::String(value) => Ok(Some(HowToCite(value.to_owned()))),
                        _ => Err(DspMetaError::ParseProject(
                            "Parse error: how_to_cite needs to be a string.".to_string(),
                        )),
                    }?;
                }
                "start_date" => {
                    start_date = match attribute.expr() {
                        Expression::String(value) => Ok(Some(StartDate(value.to_owned()))),
                        _ => Err(DspMetaError::ParseProject(
                            "Parse error: start_date needs to be a string.".to_string(),
                        )),
                    }?;
                }
                "end_date" => {
                    end_date = match attribute.expr() {
                        Expression::String(value) => Ok(Some(EndDate(value.to_owned()))),
                        _ => Err(DspMetaError::ParseProject(
                            "Parse error: end_date needs to be a string.".to_string(),
                        )),
                    }?;
                }
                "status" => {
                    status = match attribute.expr() {
                        Expression::String(value) => Ok(Some(Status::try_from(value.to_owned())?)),
                        _ => Err(DspMetaError::ParseProject(
                            "Parse error: end_date needs to be a string.".to_string(),
                        )),
                    }?;
                }
                "contact_point" => {
                    contact_point = match attribute.expr() {
                        Expression::String(value) => Ok(Some(ContactPoint(value.to_owned()))),
                        _ => Err(DspMetaError::ParseProject(
                            "Parse error: contact_point needs to be a string.".to_string(),
                        )),
                    }?;
                }
                _ => {
                    warn!("Parse error: unknown attribute '{}'.", attribute.key());
                }
            }
        }
        Ok(ExtractedProjectAttributes {
            created_at,
            created_by,
            shortcode,
            name,
            teaser_text,
            how_to_cite,
            start_date,
            end_date,
            status,
            contact_point,
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
        let result = ExtractedProjectAttributes::try_from(attributes).unwrap();
        assert_eq!(result.created_at.unwrap(), CreatedAt(1));
    }

    #[test]
    fn extract_created_by() {
        let attribute = hcl::Attribute::new("created_by", "someone");
        let attributes = vec![&attribute];
        let result = ExtractedProjectAttributes::try_from(attributes).unwrap();
        assert_eq!(result.created_by.unwrap(), CreatedBy("someone".to_owned()));
    }

    #[traced_test]
    #[test]
    fn warn_on_unknown_attribute() {
        let attribute = hcl::Attribute::new("gugus", "something");
        let attributes = vec![&attribute];
        let _ = ExtractedProjectAttributes::try_from(attributes);

        assert!(logs_contain("Parse error: unknown attribute 'gugus'"));
    }
}
