use crate::domain::value::iso_code::IsoCode;
use crate::domain::value::LangString;
use hcl::Attribute;
use std::collections::HashMap;

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
/// would be represented as:
/// ```rust
/// use dsp_meta::domain::value::Discipline;
/// use dsp_meta::domain::value::Ref;
/// use std::collections::HashMap;
/// use url::Url;
/// Discipline::Snf(Ref{
///    ref_id: "https://skos.um.es/unesco6/5501".to_string(),
///    description: "Local history".to_string(),
///    url: Url::parse("https://skos.um.es/unesco6/5501").unwrap(),
/// })
/// ```
#[derive(Debug, PartialEq)]
pub enum Discipline {
    Skos {
        ref_id: String,
        description: String,
        url: url::Url,
    },
    Snf {
        ref_id: String,
        description: String,
        url: url::Url,
    },
    Text(HashMap<IsoCode, String>),
}

impl TryFrom<Vec<&hcl::Attribute>> for Discipline {
    type Error = ();

    fn try_from(value: Vec<&Attribute>) -> Result<Self, Self::Error> {
        todo!()
    }
}
