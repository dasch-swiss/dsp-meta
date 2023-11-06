use dsp_domain::metadata::value::Shortcode;
use sophia::graph::inmem::LightGraph;
use sophia::graph::MutableGraph;
use sophia::ns::Namespace;
use sophia::term::literal::Literal;
use sophia::term::SimpleIri;

use crate::error::Result;

impl Shortcode {
    pub fn to_graph(&self, project_iri: &SimpleIri) -> Result<LightGraph> {
        let mut graph: LightGraph = LightGraph::new();

        // http://ns.dasch.swiss/repository#dsp-081C-project  http://ns.dasch.swiss/repository#hasShortcode "081C"^^http://www.w3.org/2001/XMLSchema#string
        let dsp = Namespace::new("http://ns.dasch.swiss/repository#")?;

        graph
            .insert(
                project_iri,
                &dsp.get("hasShortcode")?,
                &Literal::<String>::new_lang(&self.0, "en")?,
            )
            .expect("insert of shortcode triples into graph failed.");

        Ok(graph)
    }
}
