// placeholder for project

use sophia::graph::inmem::{FastGraph, LightGraph};
use sophia::graph::MutableGraph;
use sophia::ns::Namespace;

use crate::domain::model::entity::project::Project;
use crate::errors::DspMetaError;

impl TryInto<LightGraph> for Project {
    type Error = DspMetaError;

    fn try_into(self) -> Result<LightGraph, Self::Error> {
        let mut graph: LightGraph = LightGraph::new();

        let ex = Namespace::new("http://example.org/").map_err(|_| {
            DspMetaError::SerializeToTtl("Error serializing result to TTL.".to_string())
        })?;
        let foaf = Namespace::new("http://xmlns.com/foaf/0.1/")?;

        graph
            .insert(&ex.get("bob")?, &foaf.get("knows")?, &ex.get("alice")?)
            .map_err(|_| {
                DspMetaError::SerializeToTtl("Error serializing result to TTL.".to_string())
            })?;

        Ok(graph)
    }
}

impl TryInto<FastGraph> for Project {
    type Error = DspMetaError;

    fn try_into(self) -> Result<FastGraph, Self::Error> {
        let mut graph: FastGraph = FastGraph::new();

        let ex = Namespace::new("http://example.org/").map_err(|_| {
            DspMetaError::SerializeToTtl("Error serializing result to TTL.".to_string())
        })?;
        let foaf = Namespace::new("http://xmlns.com/foaf/0.1/")?;

        graph
            .insert(&ex.get("bob")?, &foaf.get("knows")?, &ex.get("alice")?)
            .map_err(|_| {
                DspMetaError::SerializeToTtl("Error serializing result to TTL.".to_string())
            })?;

        Ok(graph)
    }
}
