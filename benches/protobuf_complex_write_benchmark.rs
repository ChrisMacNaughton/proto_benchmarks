#[macro_use]
extern crate criterion;
extern crate capnp;
extern crate protobuf;
extern crate proto_benchmarks;

use criterion::Criterion;

use capnp::{message, serialize, message::ReaderOptions, Word};
use protobuf::{Message, parse_from_bytes};

fn simple_write(id: u64) -> Vec<u8> {
    let mut stat = proto_benchmarks::bench::Complex::new();
    stat.set_id(id);

    stat.write_to_bytes().unwrap()
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("protobuff_simple_write", |b| b.iter(|| simple_write(20)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);