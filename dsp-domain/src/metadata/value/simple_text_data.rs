use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SimpleTextData(pub String);
