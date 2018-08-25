#[macro_use]
extern crate criterion;
extern crate capnp;
extern crate protobuf;
extern crate proto_benchmarks;
extern crate quick_protobuf;

use criterion::{Criterion, Fun};

use protobuf::{Message};
use capnp::{message, serialize};
use quick_protobuf::serialize_into_vec;

fn criterion_benchmark(c: &mut Criterion) {
    // Setup capnp
    let cap = Fun::new("capnp", move |b, _i| b.iter(|| {
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

        serialize::write_message_to_words(&message)
    }));

    // Setup protobuf
    let proto = Fun::new("protobuf", move |b, _i| b.iter(||{
        let mut basic = proto_benchmarks::bench::Basic::new();
        basic.set_id(12);
        let mut stat = proto_benchmarks::bench::Complex::new();
        stat.set_basic(basic);
        stat.set_name("name".into());
        stat.set_reference("reference".into());
        stat.write_to_bytes().unwrap()
    }));

    // Setup quick-protobuf
    let quick = Fun::new("quick", move |b, _i| b.iter(|| {
        let complex = proto_benchmarks::bench_quick::Complex {
            name: "name".into(),
            reference: "reference".into(),
            basic: proto_benchmarks::bench_quick::Basic { id: 12 },
        };
        serialize_into_vec(&complex).unwrap()
    }));

    // Setup Benchmark
    let functions = vec!(cap, proto, quick);

    c.bench_functions("complex_build", functions, &20);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
