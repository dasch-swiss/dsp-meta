use std::collections::HashMap;

pub enum Term {
    Literal(String),
    Subject(String),
}

pub struct Subject {
    from_name: String,
    subject_type: String,
    identifier: String,
    identifier_prefix: String,
    properties: Vec<Property>,
}

pub struct Property {
    from_field: String,
    into_fqdn: String,
}

pub struct SerializerConfig {
    base_iri: String,
    namespaces: HashMap<String, String>,
    subjects: Vec<Subject>,
}
