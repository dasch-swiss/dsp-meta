use std::collections::HashMap;

use serde::Serialize;

use crate::domain::model::value::iso_code::IsoCode;

/// Represents multiple strings in different languages.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct LangTextData(pub HashMap<IsoCode, String>);
