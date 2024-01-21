use std::collections::HashMap;

use serde::Serialize;

use crate::metadata::value::iso_code::IsoCode;
use crate::metadata::value::lang_text_data::LangTextData;

/// A set of descriptions in different languages.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Description(pub HashMap<IsoCode, String>);

impl Default for Description {
    fn default() -> Self {
        Description(HashMap::from_iter([
            (IsoCode::DE, String::from("Die Default-Beschreibung.")),
            (IsoCode::EN, String::from("The default description.")),
            (IsoCode::FR, String::from("Le standard description.")),
        ]))
    }
}

impl From<LangTextData> for Description {
    fn from(value: LangTextData) -> Self {
        Description(value.0)
    }
}
