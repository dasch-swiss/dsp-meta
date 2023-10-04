// placeholder for project

use sophia::graph::inmem::LightGraph;
use sophia::graph::{Graph, MutableGraph};
use sophia::ns::{rdf, Namespace};

use crate::domain::model::entity::project::Project;
use crate::error::Result;

impl Project {
    pub(crate) fn as_graph(&self) -> Result<LightGraph> {
        let mut graph = LightGraph::new();

        // http://ns.dasch.swiss/repository#dsp-081C-project a http://ns.dasch.swiss/repository#Project
        let dsp = Namespace::new("http://ns.dasch.swiss/repository#")?;
        let project_iri_suffix = format!("dsp-{}-project", self.shortcode.0);
        let project_iri = dsp.get(&project_iri_suffix)?;

        let shortcode_graph = self.shortcode.as_graph(&project_iri)?;

        graph
            .insert(&project_iri, &rdf::type_, &dsp.get("Project")?)
            .expect("insert of project triples into graph failed.");

        graph
            .insert_all(shortcode_graph.triples())
            .expect("insert of project triples into graph failed.");

        Ok(graph)
    }
}
