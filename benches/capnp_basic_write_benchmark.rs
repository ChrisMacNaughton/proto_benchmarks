#[macro_use]
extern crate criterion;
extern crate capnp;
extern crate protobuf;
extern crate proto_benchmarks;

use criterion::Criterion;

use capnp::{message, serialize, Word};

fn simple_write<A: message::Allocator>(message: &message::Builder<A>) -> Vec<Word> {
    serialize::write_message_to_words(message)
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut message = message::Builder::new_default();
    {
        let mut simple = message.init_root::<proto_benchmarks::bench_capnp::basic::Builder>();
        simple.set_id(12);
    }
    c.bench_function("capnp_basic_write", |b| b.iter(|| simple_write(&message)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);