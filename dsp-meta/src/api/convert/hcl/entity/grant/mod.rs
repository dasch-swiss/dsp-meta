use dsp_domain::metadata::entity::grant::Grant;

use crate::api::convert::hcl::entity::grant::extracted_grant::ExtractedGrant;
use crate::api::convert::hcl::hcl_block::HclBlock;
use crate::error::DspMetaError;

mod extracted_grant;

impl<'a> TryInto<Grant> for HclBlock<'a> {
    type Error = DspMetaError;

    fn try_into(self) -> Result<Grant, Self::Error> {
        let mut extracted = ExtractedGrant::try_from(self)?;

        // id (1), created_at (1), created_by (1), type_of_grant (1),
        // name (0-1), number (0-1), url (0-1),  funders (1-n),

        let id = extracted.id.take().ok_or_else(|| {
            DspMetaError::ParseGrant("Parse error: Grant needs to have an id.".to_string())
        })?;

        let created_at = extracted.created_at.take().ok_or_else(|| {
            DspMetaError::ParseGrant(
                "Parse error: grant needs to have a created_at value.".to_string(),
            )
        })?;

        let created_by = extracted.created_by.take().ok_or_else(|| {
            DspMetaError::ParseGrant(
                "Parse error: grant needs to have a created_by value.".to_string(),
            )
        })?;

        let type_of_grant = extracted.type_of_grant.ok_or(DspMetaError::ParseGrant(
            "Parse error: grant needs to have a type.".to_string(),
        ))?;

        let name = extracted.name;

        let number = extracted.number;

        let url = extracted.url;

        let funders = if !extracted.funders.is_empty() {
            Ok(extracted.funders)
        } else {
            Err(DspMetaError::ParseGrant(
                "Parse grant: there needs to be at least one funder.".to_string(),
            ))
        }?;

        Ok(Grant {
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

#[cfg(test)]
mod tests {
    use dsp_domain::metadata::value::identifier::GrantId;
    use dsp_domain::metadata::value::url::Url;
    use hcl::block;
    use tracing_test::traced_test;

    use super::*;

    #[traced_test]
    #[test]
    fn test_convert_grant_block() {
        let input_block = block!(
            grant {
                id = "g1"
                created_at = 1630601300976368000u64
                created_by = "dsp-metadata-gui"
                type_of_grant = "funding"
                name = "The German Family Panel (pairfam)"
                number = "01US1706A"
                url {
                    href = "https://data.snf.ch/grants/grant/120378"
                    label = "https://data.snf.ch/grants/grant/120378"
                }
                funders = ["f1"]
            }
        );
        let grant: Grant = HclBlock(&input_block).try_into().unwrap();
        assert_eq!(grant.id, GrantId("g1".to_string()));
        assert_eq!(grant.created_at.0, 1630601300976368000u64);
        assert_eq!(grant.created_by.0, "dsp-metadata-gui".to_string());
        assert_eq!(grant.type_of_grant.0, "funding".to_string());
        assert_eq!(
            grant.name.unwrap().0,
            "The German Family Panel (pairfam)".to_string()
        );
        assert_eq!(grant.number.unwrap().0, "01US1706A".to_string());
        assert_eq!(
            grant.url,
            Some(
                Url::new(
                    "https://data.snf.ch/grants/grant/120378".to_string(),
                    "https://data.snf.ch/grants/grant/120378".to_string()
                )
                .unwrap()
            )
        );
    }
}
