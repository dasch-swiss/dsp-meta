use std::collections::HashMap;

use serde::Serialize;

use crate::metadata::value::iso_code::IsoCode;
use crate::metadata::value::lang_text_data::LangTextData;

/// A set of abstracts in different languages.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Abstract(pub HashMap<IsoCode, String>);

impl Default for Abstract {
    fn default() -> Self {
        let mut map: HashMap<IsoCode, String> = HashMap::new();
        map.insert(IsoCode::DE, String::from("Die Default-Auszug."));
        map.insert(IsoCode::EN, String::from("The default abstract."));
        map.insert(IsoCode::FR, String::from("Le standard abstract."));
        Self(map)
    }
}

impl From<LangTextData> for Abstract {
    fn from(value: LangTextData) -> Self {
        Abstract(value.0)
    }
}
