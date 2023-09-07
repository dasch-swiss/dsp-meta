use hcl::Expression;
use tracing::warn;

use crate::domain::{
    AlternativeName, CreatedAt, CreatedBy, Description, Discipline, EndDate, HowToCite, IsoCode,
    Keyword, LangString, Name, Publication, Shortcode, StartDate, TeaserText, UrlValue,
};
use crate::errors::DspMetaError;

pub struct ExtractedProjectAttributes {
    pub created_at: Option<CreatedAt>,
    pub created_by: Option<CreatedBy>,
    pub shortcode: Option<Shortcode>,
    pub name: Option<Name>,
    pub teaser_text: Option<TeaserText>,
    pub how_to_cite: Option<HowToCite>,
    pub start_date: Option<StartDate>,
    pub end_date: Option<EndDate>,
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

        for attribute in attributes {
            match attribute.key() {
                "created_at" => {
                    created_at = match attribute.expr() {
                        Expression::Number(value) => Ok(Some(CreatedAt(value.as_u64().unwrap()))), /* FIXME: get rid of unwrap */
                        _ => Err(DspMetaError::ParseProject(
                            "Parse error: created_at needs to be a number.",
                        )),
                    }?
                }
                "created_by" => {
                    created_by = match attribute.expr() {
                        Expression::String(value) => Ok(Some(CreatedBy(value.to_owned()))),
                        _ => Err(DspMetaError::ParseProject(
                            "Parse error: created_by needs to be a string.",
                        )),
                    }?
                }
                "shortcode" => {
                    shortcode = match attribute.expr() {
                        Expression::String(value) => Ok(Some(Shortcode(value.to_owned()))),
                        _ => Err(DspMetaError::ParseProject(
                            "Parse error: shortcode needs to be a string.",
                        )),
                    }?;
                }
                "name" => {
                    name = match attribute.expr() {
                        Expression::String(value) => Ok(Some(Name(value.to_owned()))),
                        _ => Err(DspMetaError::ParseProject(
                            "Parse error: name needs to be a string.",
                        )),
                    }?;
                }
                "teaser_text" => {
                    teaser_text = match attribute.expr() {
                        Expression::String(value) => Ok(Some(TeaserText(value.to_owned()))),
                        _ => Err(DspMetaError::ParseProject(
                            "Parse error: teaser_text needs to be a string.",
                        )),
                    }?;
                }
                "how_to_cite" => {
                    how_to_cite = match attribute.expr() {
                        Expression::String(value) => Ok(Some(HowToCite(value.to_owned()))),
                        _ => Err(DspMetaError::ParseProject(
                            "Parse error: how_to_cite needs to be a string.",
                        )),
                    }?;
                }
                "start_date" => {
                    start_date = match attribute.expr() {
                        Expression::String(value) => Ok(Some(StartDate(value.to_owned()))),
                        _ => Err(DspMetaError::ParseProject(
                            "Parse error: start_date needs to be a string.",
                        )),
                    }?;
                }
                "end_date" => {
                    end_date = match attribute.expr() {
                        Expression::String(value) => Ok(Some(EndDate(value.to_owned()))),
                        _ => Err(DspMetaError::ParseProject(
                            "Parse error: end_date needs to be a string.",
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
        })
    }
}

#[derive(Debug, Default, PartialEq)]
pub struct ExtractedProjectBlocks {
    pub alternative_names: Vec<AlternativeName>,
    pub description: Option<Description>,
    pub url: Option<UrlValue>,
    pub keywords: Vec<Keyword>,
    pub disciplines: Vec<Discipline>,
    pub publications: Vec<Publication>,
}

impl TryFrom<Vec<&hcl::Block>> for ExtractedProjectBlocks {
    type Error = DspMetaError;

    fn try_from(blocks: Vec<&hcl::Block>) -> Result<Self, Self::Error> {
        let mut alternative_names: Vec<AlternativeName> = vec![];
        let mut description: Option<Description> = None;
        let mut url: Option<UrlValue> = None;
        let mut keywords: Vec<Keyword> = vec![];
        let mut disciplines: Vec<Discipline> = vec![];
        let mut publications: Vec<Publication> = vec![];

        for block in blocks {
            match block.identifier.as_str() {
                "alternative_name" => {
                    alternative_names.push(AlternativeName::try_from(block)?);
                }
                "description" => {
                    description = if description.is_none() {
                        Ok(Some(Description::try_from(block)?))
                    } else {
                        Err(DspMetaError::ParseProject(
                            "Only one 'description' block allowed.",
                        ))
                    }?
                }
                "url" => {
                    url = if url.is_none() {
                        Ok(None)
                    } else {
                        Err(DspMetaError::ParseProject("Only one 'url' block allowed."))
                    }?
                }
                "keyword" => {
                    keywords.push(Keyword::try_from(block)?);
                }
                "disciplines" => {
                    disciplines = vec![];
                }
                "publications" => {
                    publications = vec![];
                }
                _ => {
                    // catch all
                    warn!("Parse error: unknown block '{}'.", block.identifier);
                }
            }
        }
        Ok(ExtractedProjectBlocks {
            alternative_names,
            description,
            url,
            keywords,
            disciplines,
            publications,
        })
    }
}

// FIXME: Where should these implementations live?
// Since these implementations convert from hcl::Block to a value object,
// would it be better to implement them as TryInto and have them live
// in a new hcl_converter module?
impl TryFrom<&hcl::Block> for AlternativeName {
    type Error = DspMetaError;

    fn try_from(block: &hcl::Block) -> Result<Self, Self::Error> {
        if block.identifier.as_str() == "alternative_name" {
            let mut values: Vec<LangString> = vec![];
            let attrs: Vec<&hcl::Attribute> = block.body.attributes().collect();
            for attr in attrs {
                values.push(LangString::try_from(attr)?)
            }
            Ok(AlternativeName::from(values))
        } else {
            // FIXME: Add received value to error message.
            Err(DspMetaError::CreateValueObject(
                "The passed block is not named correctly. Expected 'alternative_name'.",
            ))
        }
    }
}

impl TryFrom<&hcl::Block> for Description {
    type Error = DspMetaError;

    fn try_from(block: &hcl::Block) -> Result<Self, Self::Error> {
        if block.identifier.as_str() == "description" {
            let mut descriptions: Vec<LangString> = vec![];
            let attrs: Vec<&hcl::Attribute> = block.body.attributes().collect();
            for attr in attrs {
                descriptions.push(LangString::try_from(attr)?)
            }
            Ok(Description::from(descriptions))
        } else {
            Err(DspMetaError::CreateValueObject(
                "The passed block is not named correctly. Expected 'description'.",
            ))
        }
    }
}

impl TryFrom<&hcl::Block> for Keyword {
    type Error = DspMetaError;

    fn try_from(block: &hcl::Block) -> Result<Self, Self::Error> {
        if block.identifier.as_str() == "keyword" {
            let mut values: Vec<LangString> = vec![];
            let attrs: Vec<&hcl::Attribute> = block.body.attributes().collect();
            for attr in attrs {
                values.push(LangString::try_from(attr)?)
            }
            Ok(Keyword::from(values))
        } else {
            // FIXME: Add received value to error message.
            Err(DspMetaError::CreateValueObject(
                "The passed block is not named correctly. Expected 'keyword'.",
            ))
        }
    }
}

// FIXME: Where should these implementations live?
impl TryFrom<&hcl::Attribute> for LangString {
    type Error = DspMetaError;

    fn try_from(attr: &hcl::Attribute) -> Result<Self, Self::Error> {
        match attr.expr() {
            Expression::String(value) => Ok(LangString {
                iso_code: IsoCode::try_from(attr.key.as_str())?,
                string: value.to_owned(),
            }),
            _ => Err(DspMetaError::ParseProject(
                "Parse error: name needs to be a string.",
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use hcl::{block, Identifier, Number};
    use tracing_test::traced_test;
    use url::Url as UrlString;

    use super::*;
    use crate::domain::AlternativeName;

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

    #[test]
    fn extract_alternative_names() {
        let input1 = block!(
            alternative_name {
                de = "name1_de"
                en = "name1_en"
                fr = "name1_fr"
            }
        );
        let input2 = block!(
            alternative_name {
                de = "name2_de"
                en = "name2_en"
                fr = "name2_fr"
            }
        );
        let blocks = vec![&input1, &input2];
        let result = ExtractedProjectBlocks::try_from(blocks).unwrap();
        assert_eq!(result.alternative_names.len(), 2);

        let l1 = LangString {
            iso_code: IsoCode::DE,
            string: "name1_de".to_owned(),
        };
        let l2 = LangString {
            iso_code: IsoCode::EN,
            string: "name1_en".to_owned(),
        };
        let l3 = LangString {
            iso_code: IsoCode::FR,
            string: "name1_fr".to_owned(),
        };
        assert_eq!(
            result.alternative_names[0],
            AlternativeName::from(vec![l1, l2, l3])
        );
    }

    #[test]
    fn extract_description() {
        let input = block!(
            description {
                de = "descr_de"
                en = "descr_en"
                fr = "descr_fr"
            }
        );
        let blocks = vec![&input];
        let result = ExtractedProjectBlocks::try_from(blocks).unwrap();
        assert!(result.description.is_some());

        let l1 = LangString {
            iso_code: IsoCode::DE,
            string: "descr_de".to_owned(),
        };
        let l2 = LangString {
            iso_code: IsoCode::EN,
            string: "descr_en".to_owned(),
        };
        let l3 = LangString {
            iso_code: IsoCode::FR,
            string: "descr_fr".to_owned(),
        };
        assert_eq!(
            result.description.unwrap(),
            Description::from(vec![l1, l2, l3])
        );
    }

    #[test]
    fn extract_url() {
        let input = block!(
            url "https://data.dasch.swiss/dokubib/" {
                text = "Project Website"
            }
        );
        let blocks = vec![&input];
        let result = ExtractedProjectBlocks::try_from(blocks).unwrap();

        let _expected = UrlValue {
            value: UrlString::try_from("https://data.dasch.swiss/dokubib/").unwrap(),
            text: "".to_string(),
        };

        // assert!(result.url.is_some());
        // assert_eq!(
        //     result.url.unwrap(),
        //     expected
        // );
        assert!(result.url.is_none());
    }

    #[test]
    fn extract_keywords() {
        let input1 = block!(
            keyword {
                de = "keyword1_de"
                en = "keyword1_en"
                fr = "keyword1_fr"
            }

        );
        let input2 = block!(
            keyword {
                de = "keyword2_de"
                en = "keyword2_en"
                fr = "keyword2_fr"
            }
        );
        let blocks = vec![&input1, &input2];
        let result = ExtractedProjectBlocks::try_from(blocks).unwrap();
        assert_eq!(result.keywords.len(), 2);

        let l1 = LangString {
            iso_code: IsoCode::DE,
            string: "keyword1_de".to_owned(),
        };
        let l2 = LangString {
            iso_code: IsoCode::EN,
            string: "keyword1_en".to_owned(),
        };
        let l3 = LangString {
            iso_code: IsoCode::FR,
            string: "keyword1_fr".to_owned(),
        };

        assert_eq!(result.keywords[0], Keyword::from(vec![l1, l2, l3]));
    }

    #[traced_test]
    #[test]
    fn warn_on_unknown_block() {
        let block = hcl::Block::new(Identifier::new("gugus").unwrap());
        let blocks = vec![&block];
        let _ = ExtractedProjectBlocks::try_from(blocks);

        assert!(logs_contain("Parse error: unknown block 'gugus'"));
    }
}
