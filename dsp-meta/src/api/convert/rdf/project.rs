// placeholder for project

use dsp_domain::metadata::entity::project::Project;
use sophia::graph::inmem::LightGraph;
use sophia::graph::{Graph, MutableGraph};
use sophia::ns::{rdf, Namespace};

use crate::api::convert::rdf::constance::DSP_NAMESPACE_STRING;
use crate::api::convert::rdf::value::shortcode::ShortcodeDto;
use crate::error::Result;

pub struct ProjectDto(pub Project);
impl ProjectDto {
    pub(crate) fn to_graph(&self) -> Result<LightGraph> {
        let mut graph = LightGraph::new();

        // http://ns.dasch.swiss/repository#dsp-081C-project a http://ns.dasch.swiss/repository#Project
        let dsp = Namespace::new_unchecked(DSP_NAMESPACE_STRING);
        let project_iri_suffix = format!("dsp-{}-project", self.0.shortcode.0);
        let project_iri = dsp.get(&project_iri_suffix)?;

        let shortcode_graph = ShortcodeDto(&self.0.shortcode).to_graph(&project_iri)?;

        graph
            .insert(&project_iri, &rdf::type_, &dsp.get("Project")?)
            .expect("insert of project triples into graph failed.");

        graph
            .insert_all(shortcode_graph.triples())
            .expect("insert of project triples into graph failed.");

        Ok(graph)
    }
}
