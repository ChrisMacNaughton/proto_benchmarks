#[macro_use]
extern crate criterion;
extern crate capnp;
extern crate protobuf;
extern crate proto_benchmarks;

use criterion::Criterion;

use protobuf::{Message, parse_from_bytes};

fn criterion_benchmark(c: &mut Criterion) {
    let mut basic = proto_benchmarks::bench::Basic::new();
    basic.set_id(12);
    let bytes = basic.write_to_bytes().unwrap();

    c.bench_function("protobuf_simple_read", |b| b.iter(||
        parse_from_bytes::<proto_benchmarks::bench::Basic>(&bytes).unwrap()
    ));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);