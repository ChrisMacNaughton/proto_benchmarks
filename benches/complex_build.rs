#[macro_use]
extern crate criterion;
extern crate capnp;
extern crate protobuf;
extern crate proto_benchmarks;

use criterion::{Criterion, Fun};

use protobuf::{Message};

use capnp::{message, serialize};

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

    // Setup Benchmark
    let functions = vec!(cap, proto);

    c.bench_functions("complex_build", functions, &20);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);