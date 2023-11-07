use serde::Serialize;

use crate::metadata::value::simple_text_data::SimpleTextData;

#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum Publication {
    SimpleText(SimpleTextData),
}

