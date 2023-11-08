use std::collections::HashMap;

use dsp_domain::metadata::value::iso_code::IsoCode;
use dsp_domain::metadata::value::lang_text_data::LangTextData;

use crate::api::convert::hcl::hcl_attribute::HclAttributes;
use crate::error::DspMetaError;

impl<'a> TryInto<LangTextData> for HclAttributes<'a> {
    type Error = DspMetaError;

    fn try_into(self) -> Result<LangTextData, Self::Error> {
        let mut text_data: HashMap<IsoCode, String> = HashMap::new();

        for attribute in self.0 {
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
