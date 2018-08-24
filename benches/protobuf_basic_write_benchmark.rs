#[macro_use]
extern crate criterion;
extern crate capnp;
extern crate protobuf;
extern crate proto_benchmarks;

use criterion::Criterion;

use protobuf::{Message};

fn simple_write(stat: &proto_benchmarks::bench::Basic) -> Vec<u8> {
    stat.write_to_bytes().unwrap()
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut stat = proto_benchmarks::bench::Basic::new();
    stat.set_id(12);

    c.bench_function("protobuf_basic_write", |b| b.iter(|| simple_write(&stat)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);