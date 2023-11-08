use serde::Serialize;

use crate::metadata::value::Title;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Dataset {
    pub title: Title,
}
