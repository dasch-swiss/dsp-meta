use std::collections::HashMap;

pub enum Term {
    Literal(String),
    Subject(String),
}

/// A subject holds additional information for the serializer
/// to further configure how a specific rust struct should be serialized.
pub struct SubjectConfig {
    pub struct_name: String,
    pub rdf_type: String,
    pub identifier_field: String,
    pub identifier_prefix: String,
    pub properties: Vec<PropertyConfig>,
}

pub struct PropertyConfig {
    pub struct_field: String,
    pub rdf_property: String,
}

/// Serializer configuration containing mappings / instructions on how to
/// serialize rust structs into RDF. The config contains one ore more
/// `Subject`s.  
pub struct SerializerConfig {
    pub base_iri: String,
    pub namespaces: HashMap<String, String>,
    pub subjects: HashMap<String, SubjectConfig>,
}

pub struct SubjectBuilder {
    struct_name: String,
    rdf_type: String,
    identifier_field: String,
    identifier_prefix: String,
    properties: Vec<PropertyConfig>,
}
