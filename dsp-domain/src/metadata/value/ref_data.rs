use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct RefData {
    pub ref_id: String,
    pub description: String,
    pub url: url::Url,
}
