use std::collections::HashMap;

pub enum Term {
    Literal(String),
    Subject(String),
}

pub struct Subject {
    pub struct_name: String,
    pub rdf_type: String,
    pub identifier_field: String,
    pub identifier_prefix: String,
    pub properties: Vec<Property>,
}

pub struct Property {
    pub struct_field: String,
    pub rdf_property: String,
}

pub struct SerializerConfig {
    pub base_iri: String,
    pub namespaces: HashMap<String, String>,
    pub subjects: HashMap<String, Subject>,
}
