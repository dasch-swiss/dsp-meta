use clap::builder::Str;
use hcl::Expression;
use std::collections::HashMap;
use tracing::warn;

use crate::domain::value::discipline::{Discipline, DisciplineData};
use crate::domain::value::iso_code::IsoCode;
use crate::domain::value::{
    AlternativeName, ContactPoint, CreatedAt, CreatedBy, Description, EndDate, HowToCite, Keyword,
    LangString, Name, Publication, Shortcode, StartDate, TeaserText, URL,
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
        let mut contact_point: Option<ContactPoint> = None;

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
            contact_point,
        })
    }
}

#[derive(Debug, Default, PartialEq)]
pub struct ExtractedProjectBlocks {
    pub alternative_names: Vec<AlternativeName>,
    pub description: Option<Description>,
    pub url: Option<URL>,
    pub keywords: Vec<Keyword>,
    pub disciplines: Vec<Discipline>,
    pub publications: Vec<Publication>,
}

impl TryFrom<Vec<&hcl::Block>> for ExtractedProjectBlocks {
    type Error = DspMetaError;

    fn try_from(blocks: Vec<&hcl::Block>) -> Result<Self, Self::Error> {
        let mut alternative_names: Vec<AlternativeName> = vec![];
        let mut description: Option<Description> = None;
        let mut url: Option<URL> = None;
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
                            "Only one 'description' block allowed.".to_string(),
                        ))
                    }?
                }
                "url" => {
                    url = if url.is_none() {
                        Ok(Some(URL::try_from(block)?))
                    } else {
                        Err(DspMetaError::ParseProject(
                            "Only one 'url' block allowed.".to_string(),
                        ))
                    }?
                }
                "keyword" => {
                    keywords.push(Keyword::try_from(block)?);
                }
                "disciplines" => {
                    disciplines.push(Discipline::try_from(block)?);
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
            let msg = format!(
                "The passed block is not named correctly. Expected 'alternative_name', however got '{}' instead.",
                block.identifier.as_str()
            );
            Err(DspMetaError::CreateValueObject(msg))
        }
    }
}

impl TryFrom<&hcl::Block> for Description {
    type Error = DspMetaError;

    fn try_from(block: &hcl::Block) -> Result<Self, Self::Error> {
        if block.identifier.as_str() != "description" {
            let msg = format!(
                "The passed block is not named correctly. Expected 'description', however got '{}' instead.",
                block.identifier.as_str()
            );
            return Err(DspMetaError::CreateValueObject(msg));
        }

        let mut descriptions: Vec<LangString> = vec![];
        let attrs: Vec<&hcl::Attribute> = block.body.attributes().collect();
        for attr in attrs {
            descriptions.push(LangString::try_from(attr)?)
        }
        Ok(Description::from(descriptions))
    }
}

impl TryFrom<&hcl::Block> for URL {
    type Error = DspMetaError;

    fn try_from(block: &hcl::Block) -> Result<Self, Self::Error> {
        if block.identifier.as_str() != "url" {
            let msg = format!(
                "The passed block is not named correctly. Expected 'url', however got '{}' instead.",
                block.identifier.as_str()
            );
            return Err(DspMetaError::CreateValueObject(msg));
        }

        let url_value = block
            .labels
            .get(0)
            .ok_or_else(|| {
                DspMetaError::CreateValueObject(
                    "The passed url block is missing the label containing the url.".to_string(),
                )
            })?
            .as_str();

        let text_value_expr = block
            .body
            .attributes()
            .next()
            .ok_or_else(|| {
                DspMetaError::CreateValueObject(
                    "The passed url block is missing the text attribute.".to_string(),
                )
            })?
            .expr();

        let text_value = match text_value_expr {
            Expression::String(value) => Ok(value.to_owned()),
            _ => Err(DspMetaError::CreateValueObject(
                "The passed url block text attribute is not of String type.".to_string(),
            )),
        }?;

        Ok(URL {
            value: url::Url::try_from(url_value).map_err(|_| {
                DspMetaError::CreateValueObject("The passed url is not a valid url.".to_string())
            })?,
            description: text_value,
        })
    }
}

impl TryFrom<&hcl::Block> for Keyword {
    type Error = DspMetaError;

    fn try_from(block: &hcl::Block) -> Result<Self, Self::Error> {
        if block.identifier.as_str() != "keyword" {
            let msg = format!(
                "The passed block is not named correctly. Expected 'keyword', however got '{}' instead.",
                block.identifier.as_str()
            );
            return Err(DspMetaError::CreateValueObject(msg));
        }

        let mut values: Vec<LangString> = vec![];
        let attrs: Vec<&hcl::Attribute> = block.body.attributes().collect();
        for attr in attrs {
            values.push(LangString::try_from(attr)?)
        }
        Ok(Keyword::from(values))
    }
}

impl TryFrom<&hcl::Attribute> for LangString {
    type Error = DspMetaError;

    fn try_from(attr: &hcl::Attribute) -> Result<Self, Self::Error> {
        match attr.expr() {
            Expression::String(value) => Ok(LangString {
                iso_code: IsoCode::try_from(attr.key.as_str())?,
                string: value.to_owned(),
            }),
            _ => Err(DspMetaError::ParseProject(
                "Parse error: name needs to be a string.".to_string(),
            )),
        }
    }
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

        match reference_data_type.as_str() {
           "skos" || "snf" => {
               let mut ref_id: Option<String> = None;
               let mut description: Option<String> = None;
               let mut url: Option<url::Url> = None;
               let attrs: Vec<&hcl::Attribute> = block.body.attributes().collect();
               for attr in attrs {
                   match attr.key() {
                       "ref_id" => {
                           ref_id = match attr.expr() {
                               Expression::String(value) => Ok(Some(value.to_owned())),
                               _ => Err(DspMetaError::CreateValueObject(
                                   "The passed discipline block ref_id attribute is not of String type.".to_string(),
                               )),
                           }?;
                       }
                       "description" => {
                           description = match attr.expr() {
                               Expression::String(value) => Ok(Some(value.to_owned())),
                               _ => Err(DspMetaError::CreateValueObject(
                                   "The passed discipline block description attribute is not of String type.".to_string(),
                               )),
                           }?;
                       }
                       "url" => {
                           url = match attr.expr() {
                               Expression::String(value) => Ok(Some(url::Url::parse(value).map_err(|_| {
                                   DspMetaError::CreateValueObject("The passed discipline block url attribute is not a valid url.".to_string())
                               })?)),
                               _ => Err(DspMetaError::CreateValueObject(
                                   "The passed discipline block url attribute is not of String type.".to_string(),
                               )),
                           }?;
                       }
                       _ => {
                           warn!("Parse error: unknown attribute '{}'.", attr.key());
                       }
                   }
               }
               Ok(Discipline::Skos {
                   ref_id: ref_id.ok_or_else(|| {
                       DspMetaError::CreateValueObject(
                           "The passed discipline block is missing the ref_id attribute.".to_string(),
                       )
                   })?,
                   description: description.ok_or_else(|| {
                       DspMetaError::CreateValueObject(
                           "The passed discipline block is missing the description attribute.".to_string(),
                       )
                   })?,
                   url: url.ok_or_else(|| {
                       DspMetaError::CreateValueObject(
                           "The passed discipline block is missing the url attribute.".to_string(),
                       )
                   })?,
               })
           }
           "text" => {
               let mut descriptions: HashMap<IsoCode, String> = HashMap::new();
               let attrs: Vec<&hcl::Attribute> = block.body.attributes().collect();
               for attr in attrs {
                   let lang_string = LangString::try_from(attr)?;
                   descriptions.insert(lang_string.iso_code, lang_string.string);
               }
               Ok(Discipline::Text(descriptions))
           }
           _ => {
               Err(DspMetaError::CreateValueObject("The passed discipline block is missing the correct reference data type label: 'skos', 'snf', or 'text'.".to_string()))
           }
       }
    }
}

#[cfg(test)]
mod tests {
    use hcl::{block, Identifier, Number};
    use tracing_test::traced_test;

    use super::*;
    use crate::domain::value::AlternativeName;

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

        let expected = URL::new(
            "https://data.dasch.swiss/dokubib/".to_string(),
            "Project Website".to_string(),
        )
        .unwrap();

        assert!(result.url.is_some());
        assert_eq!(result.url.unwrap(), expected);
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

    #[test]
    fn extract_disciplines() {
        let input1 = block!(
            discipline skos "https://skos.um.es/unesco6/5501" {
                text = "Local history"
            }
        );
        let blocks = vec![&input1];
        let result = ExtractedProjectBlocks::try_from(blocks).unwrap();
        assert_eq!(result.disciplines.len(), 1);
    }

    #[test]
    fn extract_publications() {
        let blocks = vec![];
        let result = ExtractedProjectBlocks::try_from(blocks).unwrap();
        assert_eq!(result.publications.len(), 0);
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