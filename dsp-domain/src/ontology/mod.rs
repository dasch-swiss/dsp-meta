#![allow(dead_code)]

// Functional domain for ontologies.
//

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

#[derive(Debug, PartialEq)]
struct OntologyClass {
    name: String, // only one per class
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

struct Ontology {
    id: Iri,
    label: String,
    project_iri: Iri,
    comment: String,
}

#[derive(Debug, PartialEq)]
struct OntologyBuilder<ID, INFO> {
    id: ID,
    info: INFO,
}

/// gives us an empty ontology builder
impl OntologyBuilder<UnIdentified, NoOntologyInfo> {
    fn default() -> Self {
        Self {
            id: UnIdentified,
            info: NoOntologyInfo,
        }
    }
}

/// we always allow setting the id and info
impl OntologyBuilder<UnIdentified, NoOntologyInfo> {
    fn id(self, iri: Iri) -> OntologyBuilder<Identified, NoOntologyInfo> {
        OntologyBuilder {
            id: Identified(iri),
            info: NoOntologyInfo,
        }
    }
}

impl OntologyBuilder<Identified, NoOntologyInfo> {
    fn info(
        self,
        label: String,
        project_iri: Iri,
        comment: String,
    ) -> OntologyBuilder<Identified, WithOntologyInfo> {
        OntologyBuilder {
            id: self.id,
            info: WithOntologyInfo {
                label,
                project_iri,
                comment,
            },
        }
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
}
