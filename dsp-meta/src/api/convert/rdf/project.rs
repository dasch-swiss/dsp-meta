// placeholder for project

use dsp_domain::metadata::entity::project::Project;
use sophia::graph::inmem::LightGraph;
use sophia::graph::{Graph, MutableGraph};
use sophia::ns::{rdf, Namespace};
use sophia::term::SimpleIri;

use crate::api::convert::rdf::constance::DSP_NAMESPACE_STRING;
use crate::api::convert::rdf::value::shortcode::ShortcodeGraphDto;

pub struct ProjectGraphDto(pub Project);
impl ProjectGraphDto {
    pub(crate) fn to_graph(&self) -> LightGraph {
        let mut graph = LightGraph::new();

        // http://ns.dasch.swiss/repository#dsp-081C-project a http://ns.dasch.swiss/repository#Project
        let dsp = Namespace::new_unchecked(DSP_NAMESPACE_STRING);
        let project_iri_suffix = format!("dsp-{}-project", self.0.shortcode.0);
        let project_iri = SimpleIri::new_unchecked(dsp.as_ref(), Some(project_iri_suffix.as_str()));
        let project_class = SimpleIri::new_unchecked(dsp.as_ref(), Some("Project"));

        let shortcode_graph = ShortcodeGraphDto(&self.0.shortcode).to_graph(&project_iri);

        graph
            .insert(&project_iri, &rdf::type_, &project_class)
            .expect("insert of project triples into graph failed.");

        graph
            .insert_all(shortcode_graph.triples())
            .expect("insert of project triples into graph failed.");

        graph
    }
}
