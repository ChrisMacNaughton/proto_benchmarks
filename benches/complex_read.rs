#[macro_use]
extern crate criterion;
extern crate capnp;
extern crate proto_benchmarks;
extern crate protobuf;
extern crate prost;

use criterion::{Criterion, Fun};

use protobuf::{Message, parse_from_bytes};

use capnp::{message, serialize, message::ReaderOptions};

use prost::Message as ProstMessage;

fn criterion_benchmark(c: &mut Criterion) {
    // Setup capnp
    let mut message = message::Builder::new_default();
    {
        let mut complex = message.init_root::<proto_benchmarks::bench_capnp::complex::Builder>();
        complex.set_name("name");
        complex.set_reference("reference");

        {
            let mut basic = complex.get_basic().unwrap();
            basic.set_id(12);
        }
    }
    let words = serialize::write_message_to_words(&message);

    let cap = Fun::new("capnp", move |b, _i| b.iter(||
        serialize::read_message_from_words(&words, ReaderOptions::new()).unwrap()
    ));

    // Setup protobuf
    let mut basic = proto_benchmarks::bench::Basic::new();
    basic.set_id(12);
    let mut stat = proto_benchmarks::bench::Complex::new();
    stat.set_basic(basic);
    stat.set_name("name".into());
    stat.set_reference("reference".into());
    let bytes = stat.write_to_bytes().unwrap();
    let prost_bytes = bytes.clone();

    let proto = Fun::new("protobuf", move |b, _i| b.iter(||{
        parse_from_bytes::<proto_benchmarks::bench::Complex>(&bytes).unwrap()
    }));

    // Setup prost
    let prost = Fun::new("prost", move |b, _i| b.iter(||{
        proto_benchmarks::bench_prost::Complex::decode(&prost_bytes).unwrap()
    }));

    // Setup Benchmark
    let functions = vec!(cap, proto, prost);

    c.bench_functions("complex_read", functions, &20);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
