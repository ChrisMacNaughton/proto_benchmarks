#[macro_use]
extern crate criterion;
extern crate capnp;
extern crate protobuf;
extern crate proto_benchmarks;

use criterion::Criterion;

use capnp::{message, serialize, message::ReaderOptions, Word};
use protobuf::{Message, parse_from_bytes};

fn simple_write(n: u64) -> Vec<Word> {
    let mut message = message::Builder::new_default();
    {
        let mut simple = message.init_root::<proto_benchmarks::bench_capnp::basic::Builder>();
        simple.set_id(n);
    }

    serialize::write_message_to_words(&message)
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("capnp_simple_write", |b| b.iter(|| simple_write(20)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);