use crate::metadata::value::lang_text_data::LangTextData;
use crate::metadata::value::ref_data::RefData;

#[derive(Debug, PartialEq)]
pub enum TemporalCoverage {
    Chronontology(RefData),
    Periodo(RefData),
    Text(LangTextData),
}
