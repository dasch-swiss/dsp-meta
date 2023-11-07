use std::collections::HashMap;

use serde::Serialize;

use crate::metadata::value::iso_code::IsoCode;
use crate::metadata::value::lang_text_data::LangTextData;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct AlternativeName(pub HashMap<IsoCode, String>);

impl Default for AlternativeName {
    fn default() -> Self {
        let mut map: HashMap<IsoCode, String> = HashMap::new();
        map.insert(IsoCode::DE, String::from("Der Default AlternativeName."));
        map.insert(IsoCode::EN, String::from("The default AlternativeName."));
        map.insert(IsoCode::FR, String::from("Le default AlternativeName."));
        Self(map)
    }
}

impl From<LangTextData> for AlternativeName {
    fn from(value: LangTextData) -> Self {
        AlternativeName(value.0)
    }
}
