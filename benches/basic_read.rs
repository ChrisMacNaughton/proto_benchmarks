#[macro_use]
extern crate criterion;
extern crate capnp;
extern crate protobuf;
extern crate quick_protobuf;
extern crate proto_benchmarks;

use criterion::{Criterion, Fun};

use capnp::{message, serialize, message::ReaderOptions};
use protobuf::{Message, parse_from_bytes};
use quick_protobuf::{serialize_into_vec, deserialize_from_slice};

fn criterion_benchmark(c: &mut Criterion) {
    // Setup capnp
    let mut message = message::Builder::new_default();
    {
        let mut simple = message.init_root::<proto_benchmarks::bench_capnp::basic::Builder>();
        simple.set_id(12);
    }
    let words = serialize::write_message_to_words(&message);
    let cap = Fun::new("capnp", move |b, _i| b.iter(||
        serialize::read_message_from_words(&words, ReaderOptions::new()).unwrap()
    ));

    // Setup protobuf
    let mut basic = proto_benchmarks::bench::Basic::new();
    basic.set_id(12);
    let bytes = basic.write_to_bytes().unwrap();
    let proto = Fun::new("protobuf", move |b, _i| b.iter(||
        parse_from_bytes::<proto_benchmarks::bench::Basic>(&bytes).unwrap()
    ));

    // Setup quick-protobuf
    let basic = proto_benchmarks::bench_quick::Basic { id: 12 };
    let bytes = serialize_into_vec(&basic).unwrap();
    let quick = Fun::new("quick", move |b, _i| b.iter(|| {
        deserialize_from_slice::<proto_benchmarks::bench_quick::Basic>(&bytes).unwrap()
    }));

    // Setup Benchmark
    let functions = vec!(cap, proto, quick);

    c.bench_functions("basic_read", functions, &20);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
