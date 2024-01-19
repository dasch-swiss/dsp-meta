use serde::Serialize;

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct ProjectId(pub String);

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct DatasetId(pub String);

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct GrantId(pub String);

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct OrganizationId(pub String);

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct PersonId(pub String);
