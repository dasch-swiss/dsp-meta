use criterion::{criterion_group, criterion_main, Criterion};
use dsp_meta::domain::model::entity::project_metadata::ProjectMetadata;
use sophia::graph::inmem::LightGraph;

fn serialize() {
    let _: LightGraph = ProjectMetadata::default().try_into().unwrap();
}

fn rdf_serialization_benchmark(c: &mut Criterion) {
    c.bench_function("serialize project metadata to RDF", |b| b.iter(serialize));
}

criterion_group!(benches, rdf_serialization_benchmark);
criterion_main!(benches);
