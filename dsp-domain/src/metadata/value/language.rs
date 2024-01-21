use std::collections::HashMap;

use serde::Serialize;

use crate::metadata::value::iso_code::IsoCode;
use crate::metadata::value::lang_text_data::LangTextData;

/// A set of abstracts in different languages.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Language(pub HashMap<IsoCode, String>);

impl Default for Language {
    fn default() -> Self {
        let mut map: HashMap<IsoCode, String> = HashMap::new();
        map.insert(IsoCode::DE, String::from("Deutsch"));
        map.insert(IsoCode::EN, String::from("German"));
        map.insert(IsoCode::FR, String::from("Allemand"));
        Self(map)
    }
}

impl From<LangTextData> for Language {
    fn from(value: LangTextData) -> Self {
        Language(value.0)
    }
}
