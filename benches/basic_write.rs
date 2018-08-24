#[macro_use]
extern crate criterion;
extern crate capnp;
extern crate protobuf;
extern crate proto_benchmarks;

use criterion::{Criterion, Fun};

use capnp::{message, serialize};
use protobuf::{Message};

fn criterion_benchmark(c: &mut Criterion) {
    // Setup Capnp
    let mut message = message::Builder::new_default();
    {
        let mut simple = message.init_root::<proto_benchmarks::bench_capnp::basic::Builder>();
        simple.set_id(12);
    }
    let cap = Fun::new("capnp", move |b, _i| b.iter(||
         serialize::write_message_to_words(&message)
    ));

    // Setup protobuf
    let mut stat = proto_benchmarks::bench::Basic::new();
    stat.set_id(12);

    let proto = Fun::new("protobuf", move |b, _i| b.iter(||
        stat.write_to_bytes().unwrap()
    ));

    // Setup Benchmark
    let functions = vec!(cap, proto);

    c.bench_functions("basic_write", functions, &20);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);