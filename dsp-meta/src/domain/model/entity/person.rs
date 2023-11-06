use serde::Serialize;

#[derive(Debug, Clone, Default, PartialEq, Serialize)]
pub struct Person {
    id: String,
}
