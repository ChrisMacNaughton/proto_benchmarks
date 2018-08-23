#[macro_use]
extern crate criterion;
extern crate capnp;
extern crate protobuf;
extern crate proto_benchmarks;

use criterion::Criterion;

use protobuf::{Message};

fn complex_write(stat: &proto_benchmarks::bench::Complex) -> Vec<u8> {
    stat.write_to_bytes().unwrap()
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut basic = proto_benchmarks::bench::Basic::new();
    basic.set_id(12);
    let mut stat = proto_benchmarks::bench::Complex::new();
    stat.set_basic(basic);
    stat.set_name("name".into());
    stat.set_reference("reference".into());

    c.bench_function("protobuff_complex_write", |b| b.iter(|| complex_write(&stat)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);