use dsp_domain::metadata::value::Shortcode;
use sophia::api::graph::MutableGraph;
use sophia::api::ns::Namespace;
use sophia::inmem::graph::LightGraph;
use sophia::iri::Iri;

pub(crate) struct ShortcodeGraphDto<'a>(pub &'a Shortcode);

impl<'a> ShortcodeGraphDto<'a> {
    pub fn to_graph(&self, project_iri: &Iri<String>) -> LightGraph {
        // http://ns.dasch.swiss/repository#dsp-081C-project  http://ns.dasch.swiss/repository#hasShortcode "081C"^^http://www.w3.org/2001/XMLSchema#string
        let dsp = Namespace::new_unchecked("http://ns.dasch.swiss/repository#");

        let has_shortcode = dsp
            .get("hasShortcode")
            .expect("has_shortcode_property creation failed.");

        // mutating the graph
        let mut graph: LightGraph = LightGraph::new();

        graph
            .insert(project_iri, has_shortcode, self.0.as_string().as_str())
            .expect("triple inserted");
        graph
    }
}
