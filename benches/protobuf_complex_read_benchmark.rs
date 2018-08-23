#[macro_use]
extern crate criterion;
extern crate protobuf;
extern crate proto_benchmarks;

use criterion::Criterion;

use protobuf::{Message, parse_from_bytes};

fn criterion_benchmark(c: &mut Criterion) {
    let mut basic = proto_benchmarks::bench::Basic::new();
    basic.set_id(12);
    let mut stat = proto_benchmarks::bench::Complex::new();
    stat.set_basic(basic);
    stat.set_name("name".into());
    stat.set_reference("reference".into());
    let bytes = stat.write_to_bytes().unwrap();

    c.bench_function("protobuff_complex_read", |b| b.iter(||
        parse_from_bytes::<proto_benchmarks::bench::Complex>(&bytes).unwrap()
    ));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);