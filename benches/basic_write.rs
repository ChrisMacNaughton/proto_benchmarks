use criterion::{Criterion, Fun, criterion_group, criterion_main};
use proto_benchmarks::{bench, bench_capnp};
use proto_benchmarks::bench_generated as bench_fbs;

use capnp::{message, serialize};
use protobuf::{Message};
use flatbuffers;

fn criterion_benchmark(c: &mut Criterion) {
    // Setup Capnp
    let mut message = message::Builder::new_default();
    {
        let mut simple = message.init_root::<bench_capnp::basic::Builder>();
        simple.set_id(12);
    }
    let cap = Fun::new("capnp", move |b, _i| b.iter(||
         serialize::write_message_to_words(&message)
    ));

    // Setup protobuf
    let mut stat = bench::Basic::new();
    stat.set_id(12);

    let proto = Fun::new("protobuf", move |b, _i| b.iter(||
        stat.write_to_bytes().unwrap()
    ));

    // Setup flatbuffers
    let mut builder = flatbuffers::FlatBufferBuilder::new();
    {
        let args = bench_fbs::bench::BasicArgs{ id: 12 };
        let basic = bench_fbs::bench::Basic::create(&mut builder, &args);
        builder.finish_minimal(basic);
    }
    let fbs = Fun::new("flatbuffers", move |b, _i| b.iter(||
        // FIXME: this isn't testing much other than calling `to_vec()`.
        // The capn test above creates the builder outside, so we do the same here, but maybe builder 
        // should be "finish"ed here?
        builder.finished_data().to_vec()
    ));

    // Setup Benchmark
    let functions = vec!(cap, proto, fbs);

    c.bench_functions("basic_write", functions, &20);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);