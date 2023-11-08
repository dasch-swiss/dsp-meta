use serde::Serialize;

use crate::metadata::value::lang_text_data::LangTextData;
use crate::metadata::value::ref_data::RefData;

/// The discipline of a project can be defined in two ways:
/// 1. A reference to a discipline defined in an external reference system (e.g. SNF or SKOS)
/// 2. A text description of the discipline
///
/// Example:
/// ```hcl
/// discipline skos {
///     ref_id = "https://skos.um.es/unesco6/5501"
///     description = "Local history"
///     url = "https://skos.um.es/unesco6/5501"
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum Discipline {
    Skos(RefData),
    Snf(RefData),
    Text(LangTextData),
}
