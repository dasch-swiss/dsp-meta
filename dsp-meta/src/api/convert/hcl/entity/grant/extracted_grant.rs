use dsp_domain::metadata::value::funder::Funder;
use dsp_domain::metadata::value::identifier::GrantId;
use dsp_domain::metadata::value::url::Url;
use dsp_domain::metadata::value::{CreatedAt, CreatedBy, GrantNumber, GrantType, Name};
use hcl::Expression;
use log::warn;

use crate::api::convert::hcl::hcl_block::HclBlock;
use crate::error::DspMetaError;

pub struct ExtractedGrant {
    pub id: Option<GrantId>,
    pub created_at: Option<CreatedAt>,
    pub created_by: Option<CreatedBy>,
    pub type_of_grant: Option<GrantType>,
    pub name: Option<Name>,
    pub number: Option<GrantNumber>,
    pub url: Option<Url>,
    pub funders: Vec<Funder>,
}

impl<'a> TryFrom<HclBlock<'a>> for ExtractedGrant {
    type Error = DspMetaError;

    fn try_from(hcl_block: HclBlock<'a>) -> Result<Self, Self::Error> {
        let mut id: Option<GrantId> = None;
        let mut created_at: Option<CreatedAt> = None;
        let mut created_by: Option<CreatedBy> = None;
        let mut type_of_grant: Option<GrantType> = None;
        let mut name: Option<Name> = None;
        let mut number: Option<GrantNumber> = None;
        let mut url: Option<Url> = None;
        let mut funders: Vec<Funder> = Vec::new();

        // check if the block is named grant
        if hcl_block.0.identifier.as_str() != "grant" {
            return Err(DspMetaError::ParseGrant(
                format!(
                    "Parse error: grant block needs to be named 'grant', however got '{}' instead.",
                    hcl_block.0.identifier.as_str()
                )
                .to_string(),
            ));
        }

        // extract the attributes
        // id (1), created_at (1), created_by (1), type_of_grant (1),
        // name (0-1), number (0-1), funders (1-n),

        let attributes: Vec<&hcl::Attribute> = hcl_block.0.body.attributes().collect();

        for attribute in attributes {
            match attribute.key() {
                "id" => {
                    if id.is_some() {
                        return Err(DspMetaError::ParseGrant(
                            "Parse error: grant needs to have only one id.".to_string(),
                        ));
                    }
                    id = match attribute.expr() {
                        hcl::Expression::String(value) => Some(GrantId(value.to_string())),
                        _ => {
                            return Err(DspMetaError::ParseGrant(
                                "Parse error: grant id needs to be a string.".to_string(),
                            ))
                        }
                    };
                }
                "created_at" => {
                    if created_at.is_some() {
                        return Err(DspMetaError::ParseGrant(
                            "Parse error: grant needs to have only one created_at value."
                                .to_string(),
                        ));
                    }
                    created_at = match attribute.expr() {
                        hcl::Expression::Number(value) => Some(CreatedAt(value.as_u64().unwrap())),
                        _ => {
                            return Err(DspMetaError::ParseGrant(
                                "Parse error: grant created_at needs to be a number.".to_string(),
                            ))
                        }
                    }
                }
                "created_by" => {
                    if created_by.is_some() {
                        return Err(DspMetaError::ParseGrant(
                            "Parse error: grant needs to have only one created_by value."
                                .to_string(),
                        ));
                    }
                    created_by = match attribute.expr() {
                        hcl::Expression::String(value) => Some(CreatedBy(value.to_string())),
                        _ => {
                            return Err(DspMetaError::ParseGrant(
                                "Parse error: grant created_by needs to be a string.".to_string(),
                            ))
                        }
                    }
                }
                "type_of_grant" => {
                    if type_of_grant.is_some() {
                        return Err(DspMetaError::ParseGrant(
                            "Parse error: grant needs to have only one type_of_grant value."
                                .to_string(),
                        ));
                    }
                    type_of_grant = match attribute.expr() {
                        hcl::Expression::String(value) => Some(GrantType(value.to_string())),
                        _ => {
                            return Err(DspMetaError::ParseGrant(
                                "Parse error: grant type_of_grant needs to be a string."
                                    .to_string(),
                            ))
                        }
                    }
                }
                "name" => {
                    if name.is_some() {
                        return Err(DspMetaError::ParseGrant(
                            "Parse error: grant needs to have only one name value.".to_string(),
                        ));
                    }
                    name = match attribute.expr() {
                        hcl::Expression::String(value) => Some(Name(value.to_string())),
                        _ => {
                            return Err(DspMetaError::ParseGrant(
                                "Parse error: grant name needs to be a string.".to_string(),
                            ))
                        }
                    }
                }
                "number" => {
                    if number.is_some() {
                        return Err(DspMetaError::ParseGrant(
                            "Parse error: grant needs to have only one number value.".to_string(),
                        ));
                    }
                    number = match attribute.expr() {
                        hcl::Expression::String(value) => Some(GrantNumber(value.to_string())),
                        _ => {
                            return Err(DspMetaError::ParseGrant(
                                "Parse error: grant number needs to be a string.".to_string(),
                            ))
                        }
                    }
                }
                "funders" => {
                    match attribute.expr() {
                        Expression::Array(values) => {
                            for value in values {
                                match value {
                                    Expression::String(value) => {
                                        // pushing the value to the vector
                                        funders.push(Funder(value.to_string()))
                                    }
                                    _ => {
                                        return Err(DspMetaError::ParseGrant(
                                            "Parse error: grant funder needs to be a string."
                                                .to_string(),
                                        ))
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(DspMetaError::ParseGrant(
                                "Parse error: grant funder needs to be an array.".to_string(),
                            ))
                        }
                    }
                }
                _ => warn!("Parse error: unknown attribute '{}'.", attribute.key()),
            }
        }

        // extract the blocks
        // url (0-1),

        let blocks: Vec<&hcl::Block> = hcl_block.0.body.blocks().collect();

        for block in blocks {
            match block.identifier.as_str() {
                "url" => {
                    if url.is_some() {
                        return Err(DspMetaError::ParseGrant(
                            "Parse error: grant needs to have only one url.".to_string(),
                        ));
                    }
                    url = Some(HclBlock(block).try_into()?)
                }
                _ => warn!(
                    "Parse error: unknown or not implemented block '{}'.",
                    block.identifier
                ),
            }
        }

        Ok(Self {
            id,
            created_at,
            created_by,
            type_of_grant,
            name,
            number,
            url,
            funders,
        })
    }
}
