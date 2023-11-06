use serde::Serialize;

#[derive(Debug, Default, Clone, PartialEq, Serialize)]
pub struct Version(pub u64);
