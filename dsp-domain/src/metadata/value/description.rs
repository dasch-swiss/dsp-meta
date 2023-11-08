use std::collections::HashMap;

use serde::Serialize;

use crate::metadata::value::iso_code::IsoCode;
use crate::metadata::value::lang_text_data::LangTextData;

/// A set of descriptions in different languages.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Description(pub HashMap<IsoCode, String>);

impl Default for Description {
    fn default() -> Self {
        let mut map: HashMap<IsoCode, String> = HashMap::new();
        map.insert(IsoCode::DE, String::from("Die Default-Beschreibung."));
        map.insert(IsoCode::EN, String::from("The default description."));
        map.insert(IsoCode::FR, String::from("Le standard description."));
        Self(map)
    }
}

impl From<LangTextData> for Description {
    fn from(value: LangTextData) -> Self {
        Description(value.0)
    }
}