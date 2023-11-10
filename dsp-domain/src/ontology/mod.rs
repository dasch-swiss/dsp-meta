#![allow(dead_code)]

// Functional domain for ontologies.
//

use std::cell::RefCell;
use std::collections::{HashMap, HashSet};

use crate::error::DspDomainError;

type Iri = String;

#[derive(Debug, PartialEq)]
pub struct Identified(pub Iri);

#[derive(Debug, PartialEq)]
struct UnIdentified;

#[derive(Debug, PartialEq)]
struct WithOntologyInfo {
    label: String,
    project_iri: Iri,
    comment: String,
}

#[derive(Debug, PartialEq)]
struct NoOntologyInfo;

/// Used to track defined classes inside the ontology
struct DefinedClasses(RefCell<HashSet<Iri>>);

#[derive(Debug, Clone, PartialEq)]
struct OntologyClass {
    id: Iri, // only one per ontology
    label: String,
    comment: String,
}

#[derive(Debug, PartialEq)]
struct OntologyProperty {
    name: Iri, // only one per ontology
    label: String,
    comment: String,
    range: String, // what type is allowed in object position: subject / predicate / object
}

#[derive(Debug, PartialEq)]
struct Cardinality {
    ontology_class: OntologyClass,
    ontology_property: OntologyProperty,
    cardinality_type: CardinalityType,
}

#[derive(Debug, PartialEq)]
enum CardinalityType {
    MaxCardinalityOne,
    MinCardinalityOne,
    MinCardinalityZero,
}

pub struct Ontology {
    id: Iri,
    label: String,
    project_iri: Iri,
    comment: String,
}

#[derive(Debug, PartialEq)]
struct OntologyBuilder<ID, INFO> {
    id: ID,
    info: INFO,
    classes: HashMap<Iri, OntologyClass>,
}

/// gives us an empty ontology builder
impl OntologyBuilder<UnIdentified, NoOntologyInfo> {
    fn default() -> Self {
        Self {
            id: UnIdentified,
            info: NoOntologyInfo,
            classes: HashMap::new(),
        }
    }
}

/// we always allow setting the id and info
impl<T> OntologyBuilder<UnIdentified, T> {
    fn id(self, iri: Iri) -> OntologyBuilder<Identified, T> {
        OntologyBuilder {
            id: Identified(iri),
            info: self.info,
            classes: self.classes,
        }
    }
}

impl<T> OntologyBuilder<T, NoOntologyInfo> {
    fn info(
        self,
        label: String,
        project_iri: Iri,
        comment: String,
    ) -> OntologyBuilder<T, WithOntologyInfo> {
        OntologyBuilder {
            id: self.id,
            info: WithOntologyInfo {
                label,
                project_iri,
                comment,
            },
            classes: self.classes,
        }
    }
}

impl<T, S> OntologyBuilder<T, S> {
    fn add_class(mut self, clazz: OntologyClass) -> Result<OntologyBuilder<T, S>, DspDomainError> {
        if self.classes.contains_key(&clazz.id) {
            return Err(DspDomainError::CreateDomainObject);
        }
        let id = clazz.id.clone();
        self.classes.insert(id, clazz);
        Ok(self)
    }
}

/// we can only construct the ontology once all needed information is set
impl OntologyBuilder<Identified, WithOntologyInfo> {
    fn into_ontology(self) -> Ontology {
        Ontology {
            id: self.id.0,
            label: self.info.label,
            project_iri: self.info.project_iri,
            comment: self.info.comment,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_empty_ontology() {
        let empty = OntologyBuilder::default();
        assert_eq!(empty.id, UnIdentified);
        assert_eq!(empty.info, NoOntologyInfo);
    }

    #[test]
    fn ontology_with_id() {
        let empty = OntologyBuilder::default();
        let with_id = empty.id("https:://example.com/example".to_owned());
        assert_eq!(
            with_id.id,
            Identified("https:://example.com/example".to_owned())
        );
        assert_eq!(with_id.info, NoOntologyInfo);
    }

    #[test]
    fn ontology_with_info() {
        let empty = OntologyBuilder::default();

        let with_id = empty.id("https:://example.com/example".to_owned());
        let with_info = with_id.info("label".to_owned(), "iri".to_owned(), "comment".to_owned());
        assert_eq!(
            with_info.id,
            Identified("https:://example.com/example".to_owned())
        );
        assert_eq!(
            with_info.info,
            WithOntologyInfo {
                label: "label".to_owned(),
                project_iri: "iri".to_owned(),
                comment: "comment".to_owned(),
            }
        )
    }

    #[test]
    fn ontology_with_info_1() {
        let empty = OntologyBuilder::default();
        let with_info = empty.info("label".to_owned(), "iri".to_owned(), "comment".to_owned());
        let with_id = with_info.id("https:://example.com/example".to_owned());

        assert_eq!(
            with_id.id,
            Identified("https:://example.com/example".to_owned())
        );
        assert_eq!(
            with_id.info,
            WithOntologyInfo {
                label: "label".to_owned(),
                project_iri: "iri".to_owned(),
                comment: "comment".to_owned(),
            }
        )
    }

    #[test]
    fn ontology_with_class() {
        let empty = OntologyBuilder::default();
        let actual = empty
            .info("label".to_owned(), "iri".to_owned(), "comment".to_owned())
            .id("https:://example.com/example".to_owned())
            .add_class(OntologyClass {
                id: "something".to_string(),
                label: "".to_string(),
                comment: "".to_string(),
            });

        assert!(actual.unwrap().classes.contains_key("something"));
    }

    #[test]
    fn error_adding_duplicate_class() {
        let empty = OntologyBuilder::default();
        let actual = empty
            .add_class(OntologyClass {
                id: "something".to_string(),
                label: "".to_string(),
                comment: "".to_string(),
            })
            .unwrap()
            .add_class(OntologyClass {
                id: "something".to_string(),
                label: "".to_string(),
                comment: "".to_string(),
            });

        assert!(actual.is_err());
    }
}
