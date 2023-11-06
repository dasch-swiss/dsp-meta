use std::collections::HashMap;

use serde::Serialize;

use crate::domain::model::value::iso_code::IsoCode;

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
