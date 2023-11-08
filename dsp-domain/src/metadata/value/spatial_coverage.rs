use crate::metadata::value::ref_data::RefData;

#[derive(Debug, PartialEq)]
pub enum SpacialCoverage {
    Geonames(RefData),
}
