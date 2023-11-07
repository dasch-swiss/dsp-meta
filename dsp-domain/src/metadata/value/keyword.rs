use std::collections::HashMap;

use serde::Serialize;

use crate::metadata::value::iso_code::IsoCode;
use crate::metadata::value::lang_text_data::LangTextData;

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
pub struct Keyword(pub HashMap<IsoCode, String>);

impl From<LangTextData> for Keyword {
    fn from(value: LangTextData) -> Self {
        Keyword(value.0)
    }
}
