use sophia::graph::inmem::LightGraph;
use sophia::graph::*;
use sophia::ns::Namespace;
use sophia::serializer::nt::NtSerializer;
use sophia::serializer::*;
use tracing::trace;

use crate::domain::model::entity::project_metadata::ProjectMetadata;
use crate::error::DspMetaError;

/// Convert `ProjectMetadata` into a TTL response.
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
/// LightGraph (in contrast to FastGraph) is a simple in-memory graph implementation with a low
/// memory footprint, without indexing, thus fast to build but slow to query. Since we are only
/// interested in building the graph and immediately serializing it, this is the better choice.
///
/// Benchmarking results support this decision.
impl TryInto<LightGraph> for ProjectMetadata {
    type Error = DspMetaError;

    fn try_into(self) -> Result<LightGraph, Self::Error> {
        trace!("entered ProjectMetadata::to_turtle()");

        let mut graph: LightGraph = LightGraph::new();

        let _dsp = Namespace::new("http://ns.dasch.swiss/repository#").map_err(|_| {
            DspMetaError::SerializeToRdf("Error serializing result to TTL.".to_string())
        })?;

        let _prov = Namespace::new("http://www.w3.org/ns/prov#").map_err(|_| {
            DspMetaError::SerializeToRdf("Error serializing result to TTL.".to_string())
        })?;

        let _sdo = Namespace::new("https://schema.org/").map_err(|_| {
            DspMetaError::SerializeToRdf("Error serializing result to TTL.".to_string())
        })?;

        let _xsd = Namespace::new("http://www.w3.org/2001/XMLSchema#").map_err(|_| {
            DspMetaError::SerializeToRdf("Error serializing result to TTL.".to_string())
        })?;

        let project_graph = self.project.as_graph()?;

        graph
            .insert_all(project_graph.triples())
            .expect("insert of project triples into project metadata graph failed");

        let mut nt_stringifier = NtSerializer::new_stringifier();
        let example2 = nt_stringifier
            .serialize_graph(&graph)
            .map_err(|_| {
                DspMetaError::SerializeToRdf("Error serializing result to TTL.".to_string())
            })?
            .as_str();
        trace!("The resulting graph\n{}", example2);
        Ok(graph)
    }
}
