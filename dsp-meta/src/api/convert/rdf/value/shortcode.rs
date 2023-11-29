use dsp_domain::metadata::value::Shortcode;
use sophia::graph::inmem::LightGraph;
use sophia::graph::MutableGraph;
use sophia::ns::Namespace;
use sophia::term::literal::Literal;
use sophia::term::SimpleIri;

pub(crate) struct ShortcodeGraphDto<'a>(pub &'a Shortcode);

impl<'a> ShortcodeGraphDto<'a> {
    pub fn to_graph(&self, project_iri: &SimpleIri) -> LightGraph {
        let mut graph: LightGraph = LightGraph::new();

        // http://ns.dasch.swiss/repository#dsp-081C-project  http://ns.dasch.swiss/repository#hasShortcode "081C"^^http://www.w3.org/2001/XMLSchema#string
        let dsp = Namespace::new_unchecked("http://ns.dasch.swiss/repository#");
        let has_shortcode_property = SimpleIri::new_unchecked(dsp.as_ref(), Some("hasShortcode"));

        graph
            .insert(
                project_iri,
                &has_shortcode_property,
                &Literal::<String>::new_lang_unchecked(&self.0.as_string(), "en"),
            )
            .expect("insert of shortcode triples into graph failed.");

        graph
    }
}
