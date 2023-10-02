use sophia::graph::inmem::FastGraph;
use sophia::graph::*;
use sophia::ns::Namespace;
use sophia::parser::turtle;
use sophia::serializer::nt::NtSerializer;
use sophia::serializer::*;
use sophia::triple::stream::TripleSource;
use tracing::trace;

use crate::domain::model::entity::project_metadata::ProjectMetadata;
use crate::errors::DspMetaError;

impl ProjectMetadata {
    pub fn to_turtle(&self) -> Result<String, DspMetaError> {
        trace!("entered ProjectMetadata::to_turtle()");
        let example = r#"
    @prefix : <http://example.org/>.
    @prefix foaf: <http://xmlns.com/foaf/0.1/>.

    :alice foaf:name "Alice";
           foaf:mbox <mailto:alice@work.example> .

    :bob foaf:name "Bob".
"#;
        let mut graph: FastGraph = turtle::parse_str(example).collect_triples().map_err(|_| {
            DspMetaError::SerializeToTtl("Error serializing result to TTL.".to_string())
        })?;

        let ex = Namespace::new("http://example.org/").map_err(|_| {
            DspMetaError::SerializeToTtl("Error serializing result to TTL.".to_string())
        })?;
        let foaf = Namespace::new("http://xmlns.com/foaf/0.1/")?;
        graph
            .insert(&ex.get("bob")?, &foaf.get("knows")?, &ex.get("alice")?)
            .map_err(|_| {
                DspMetaError::SerializeToTtl("Error serializing result to TTL.".to_string())
            })?;
        let mut nt_stringifier = NtSerializer::new_stringifier();
        let example2 = nt_stringifier
            .serialize_graph(&graph)
            .map_err(|_| {
                DspMetaError::SerializeToTtl("Error serializing result to TTL.".to_string())
            })?
            .as_str();
        trace!("The resulting graph\n{}", example2);
        Ok(example2.to_string())
    }
}
