// placeholder for project

use dsp_domain::metadata::entity::project::Project;
use sophia::api::graph::{Graph, MutableGraph};
use sophia::api::ns::{rdf, Namespace};
use sophia::inmem::graph::LightGraph;
use sophia::iri::Iri;

use crate::api::convert::rdf::constance::DSP_NAMESPACE_STRING;
use crate::api::convert::rdf::value::shortcode::ShortcodeGraphDto;

pub struct ProjectGraphDto(pub Project);
impl ProjectGraphDto {
    pub(crate) fn to_graph(&self) -> LightGraph {
        let mut graph = LightGraph::new();

        // http://ns.dasch.swiss/repository#dsp-081C-project a http://ns.dasch.swiss/repository#Project
        let ns = Namespace::new_unchecked(DSP_NAMESPACE_STRING);

        let project_iri = ns
            .get(format!("{}dsp-{}-project", ns.as_str(), self.0.shortcode.0).as_str())
            .expect("project_iri creation failed.")
            .to_string();
        let project_iri = Iri::new(project_iri).expect("project_iri creation failed.");

        let project_class = ns.get("Project").expect("project_class creation failed.");

        graph
            .insert(&project_iri, rdf::type_, project_class)
            .expect("insert of project triples into graph failed.");

        let shortcode_graph = ShortcodeGraphDto(&self.0.shortcode).to_graph(&project_iri);
        graph
            .insert_all(shortcode_graph.triples())
            .expect("insert of project triples into graph failed.");

        graph
    }
}
