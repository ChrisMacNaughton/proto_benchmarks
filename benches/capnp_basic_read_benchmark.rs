#[macro_use]
extern crate criterion;
extern crate capnp;
extern crate protobuf;
extern crate proto_benchmarks;

use criterion::Criterion;

use capnp::{message, serialize, message::ReaderOptions};

fn criterion_benchmark(c: &mut Criterion) {
    let mut message = message::Builder::new_default();
    {
        let mut simple = message.init_root::<proto_benchmarks::bench_capnp::basic::Builder>();
        simple.set_id(12);
    }
    let words = serialize::write_message_to_words(&message);

    c.bench_function("capnp_basic_read", |b| b.iter(||
        serialize::read_message_from_words(&words, ReaderOptions::new()).unwrap()
    ));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);