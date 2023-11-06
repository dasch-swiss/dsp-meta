use std::collections::HashMap;

use serde::Serialize;

use crate::metadata::value::iso_code::IsoCode;

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
pub struct Keyword(pub HashMap<IsoCode, String>);
