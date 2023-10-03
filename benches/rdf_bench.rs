use criterion::{criterion_group, criterion_main, Criterion};
use dsp_meta::domain::model::entity::project_metadata::ProjectMetadata;
use sophia::graph::inmem::{FastGraph, LightGraph};

fn serialize_with_fast_graph() {
    let _: FastGraph = ProjectMetadata::default().try_into().unwrap();
}

fn serialize_with_light_graph() {
    let _: LightGraph = ProjectMetadata::default().try_into().unwrap();
}

fn rdf_serialization_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("RDF Serialization");
    group.bench_function("fast graph", |b| b.iter(serialize_with_fast_graph));
    group.bench_function("light graph", |b| b.iter(serialize_with_light_graph));
    group.finish();
}

criterion_group!(benches, rdf_serialization_benchmark);
criterion_main!(benches);
