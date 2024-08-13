use dsp_domain::metadata::entity::project_metadata::ProjectMetadata;
use sophia::api::ns::Namespace;
use sophia::api::prefix::{Prefix, PrefixMapPair};
use sophia::api::prelude::Stringifier;
use sophia::api::serializer::TripleSerializer;
use sophia::inmem::graph::LightGraph;
use sophia::iri::Iri;
use sophia::turtle::serializer::turtle::{TurtleConfig, TurtleSerializer};
use tracing::trace;

use crate::api::convert::rdf::constance::{
    DSP_NAMESPACE_STRING, PROV_NAMESPACE_STRING, SCHEMA_NAMESPACE_STRING, XSD_NAMESPACE_STRING,
};
use crate::api::convert::rdf::project::ProjectGraphDto;
pub struct ProjectMetadataGraph {
    graph: LightGraph,
}

impl ProjectMetadataGraph {
    /// Serialize the graph to a turtle string.
    ///
    /// Example output:
    /// ```turtle
    /// @prefix dsp: <http://ns.dasch.swiss/repository#> .
    /// @prefix prov: <http://www.w3.org/ns/prov#> .
    /// @prefix sdo: <https://schema.org/> .
    /// @prefix xsd: <http://www.w3.org/2001/XMLSchema#> .
    ///
    /// dsp:dsp-081C-project a dsp:Project ;
    ///     dsp:hasDataset dsp:dsp-081C-dataset-000 ;
    ///     dsp:hasDescription "The database documents the events that took place in the Hôtel de Musique
    /// in Bern between 1766 and 1905. The repertoire was constituted by different kinds of spectacles
    /// like theatre plays, operas, ballets, concerts, dance parties, acrobatic performances, conferences
    /// or magicians. The list reconstructs the lifely and colourful theatre culture of Bern in the 19th
    /// Century."@en ;     dsp:hasDiscipline "10302 Schweizer Geschichte"@de,
    ///         "10405 Musikologie"@de,
    ///         "10406 Theater-und Filmwissenschaften"@de,
    ///         "10604 Musik und Theater"@de ;
    ///     dsp:hasFunder dsp:dsp-081C-organization-000 ;
    ///     dsp:hasHowToCite "HdM-Bern"^^xsd:string ;
    ///     dsp:hasKeyword "Bern"@de,
    ///         "19 Century"@en,
    ///         "Concert"@en,
    ///         "Music"@en,
    ///         "Musicology"@en,
    ///         "Opera"@en,
    ///         "Spectales"@en,
    ///         "Switzerland"@en,
    ///         "Theater history"@en,
    ///         "Theatre"@en ;
    ///     dsp:hasName "Hôtel de Musique Bern"^^xsd:string ;
    ///     dsp:hasShortcode "081C"^^xsd:string ;
    ///     dsp:hasSpatialCoverage [ a sdo:URL ;
    ///             sdo:propertyID [ a sdo:PropertyValue ;
    ///                     sdo:propertyID "Bern" ] ;
    ///             sdo:url "https://www.geonames.org/2661552" ] ;
    ///     dsp:hasStartDate "2009-04-01"^^xsd:date ;
    ///     dsp:hasTeaser "The database documents the different kinds of spectacles such as theatre
    /// plays, operas, ballets, or concerts that took place in the Hôtel de Musique in Bern between 1766
    /// and 1905."^^xsd:string ;     dsp:hasTemporalCoverage [ a sdo:URL ;
    ///             sdo:propertyID [ a sdo:PropertyValue ;
    ///                     sdo:propertyID "Sonderbund, 1845-1847" ] ;
    ///             sdo:url "http://n2t.net/ark:/99152/p06c6g3p4cf" ],
    ///         [ a sdo:URL ;
    ///             sdo:propertyID [ a sdo:PropertyValue ;
    ///                     sdo:propertyID "Under Mediation act, 1803-1814" ] ;
    ///             sdo:url "http://n2t.net/ark:/99152/p06c6g3pvr5" ],
    ///         [ a sdo:URL ;
    ///             sdo:propertyID [ a sdo:PropertyValue ;
    ///                     sdo:propertyID "Helvetic Republic, 1798-1803" ] ;
    ///             sdo:url "http://n2t.net/ark:/99152/p06c6g364np" ],
    ///         "1766-1905"@de,
    ///         "1766-1905"@en,
    ///         "1766-1905"@fr ;
    ///     dsp:hasURL [ a sdo:URL ;
    ///             sdo:propertyID [ a sdo:PropertyValue ;
    ///                     sdo:propertyID "Discover Project Data" ] ;
    ///             sdo:url "https://admin.dasch.swiss/project/081C" ] .
    /// ```
    pub fn to_turtle_string(&self) -> String {
        let prefix_map: Vec<PrefixMapPair> = vec![
            (
                Prefix::new_unchecked("dsp".into()),
                Iri::new_unchecked("http://ns.dasch.swiss/repository#".into()),
            ),
            (
                Prefix::new_unchecked("prov".into()),
                Iri::new_unchecked("http://www.w3.org/ns/prov#".into()),
            ),
            (
                Prefix::new_unchecked("sdo".into()),
                Iri::new_unchecked("https://schema.org/".into()),
            ),
            (
                Prefix::new_unchecked("rdf".into()),
                Iri::new_unchecked("http://www.w3.org/1999/02/22-rdf-syntax-ns#".into()),
            ),
            (
                Prefix::new_unchecked("rdfs".into()),
                Iri::new_unchecked("http://www.w3.org/2000/01/rdf-schema#".into()),
            ),
            (
                Prefix::new_unchecked("xsd".into()),
                Iri::new_unchecked("http://www.w3.org/2001/XMLSchema#".into()),
            ),
        ];

        let config = TurtleConfig::new()
            .with_pretty(true)
            .with_own_prefix_map(prefix_map);
        let mut serializer = TurtleSerializer::new_stringifier_with_config(config);
        let out = serializer
            .serialize_graph(&self.graph)
            .expect("Error serializing graph to turtle.")
            .to_string();
        out
    }
}

/// A wrapper around an optional ProjectMetadata.
pub struct ProjectMetadataGraphWrapper(pub ProjectMetadata);

/// Convert a `ProjectMetadataGraphDto` into a `ProjectMetadataGraph`.
///
/// The underlying graph implementation is a `LightGraph` (in contrast to FastGraph) which is a
/// simple in-memory graph graph implementation with a low memory footprint, without indexing,
/// thus fast to build but slow to query. Since we are only interested in building the graph and
/// immediately serializing it, this is the better choice (supported by benchmarking results).
impl From<ProjectMetadataGraphWrapper> for ProjectMetadataGraph {
    fn from(value: ProjectMetadataGraphWrapper) -> ProjectMetadataGraph {
        trace!("entered ProjectMetadataGraph::from()");
        let _dsp = Namespace::new_unchecked(DSP_NAMESPACE_STRING);

        let _prov = Namespace::new_unchecked(PROV_NAMESPACE_STRING);

        let _sdo = Namespace::new_unchecked(SCHEMA_NAMESPACE_STRING);

        let _xsd = Namespace::new_unchecked(XSD_NAMESPACE_STRING);

        let graph = ProjectGraphDto(value.0.project).to_graph();
        let result = ProjectMetadataGraph { graph };

        trace!("The resulting graph\n{}", result.to_turtle_string());

        result
    }
}
