use std::collections::HashMap;

use dsp_domain::metadata::value::iso_code::IsoCode;
use dsp_domain::metadata::value::lang_text_data::LangTextData;

use crate::error::DspMetaError;

impl TryFrom<Vec<&hcl::Attribute>> for LangTextData {
    type Error = DspMetaError;

    fn try_from(attributes: Vec<&hcl::Attribute>) -> Result<Self, Self::Error> {
        let mut text_data: HashMap<IsoCode, String> = HashMap::new();

        for attribute in attributes {
            let iso_code = IsoCode::try_from(attribute.key())?;
            let text = match attribute.expr() {
                hcl::Expression::String(value) => Ok(value.to_owned()),
                _ => Err(DspMetaError::CreateValueObject(
                    "The attribute value is not of String type.".to_string(),
                )),
            }?;

            text_data.insert(iso_code, text);
        }

        Ok(LangTextData(text_data))
    }
}
