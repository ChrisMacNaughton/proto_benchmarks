use criterion::{Criterion, Fun, criterion_group, criterion_main};
use proto_benchmarks::{bench, bench_capnp};
use proto_benchmarks::bench_generated as bench_fbs;

use capnp::{message, serialize, message::ReaderOptions};
use protobuf::{Message, parse_from_bytes};
use flatbuffers;

fn criterion_benchmark(c: &mut Criterion) {
    // Setup capnp
    let mut message = message::Builder::new_default();
    {
        let mut complex = message.init_root::<bench_capnp::complex::Builder>();
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
    let mut basic = bench::Basic::new();
    basic.set_id(12);
    let mut stat = bench::Complex::new();
    stat.set_basic(basic);
    stat.set_name("name".into());
    stat.set_reference("reference".into());
    let bytes = stat.write_to_bytes().unwrap();

    let proto = Fun::new("protobuf", move |b, _i| b.iter(||{
        parse_from_bytes::<bench::Complex>(&bytes).unwrap()
    }));

    // Set flatbuffers
    let mut builder = flatbuffers::FlatBufferBuilder::new();
    {
        let args = bench_fbs::bench::BasicArgs{id: 12};
        let basic = Some(bench_fbs::bench::Basic::create(&mut builder, &args));
        let name = Some(builder.create_string("name"));
        let reference = Some(builder.create_string("reference"));
        let args = bench_fbs::bench::ComplexArgs{ basic, name, reference };
        let complex = bench_fbs::bench::Complex::create(&mut builder, &args);
        builder.finish_minimal(complex);
    }
    let fbs = Fun::new("flatbuffers", move |b, _i| b.iter(||{
        flatbuffers::get_root::<bench_fbs::bench::Complex>(builder.finished_data())
    }));

    // Setup Benchmark
    let functions = vec!(cap, proto, fbs);

    c.bench_functions("complex_read", functions, &20);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);