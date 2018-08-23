#[macro_use]
extern crate criterion;
extern crate capnp;
extern crate protobuf;
extern crate proto_benchmarks;

use criterion::Criterion;

use capnp::{message, serialize, Word};

fn complex_write<A: message::Allocator>(message: &message::Builder<A>) -> Vec<Word> {
    serialize::write_message_to_words(message)
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut message = message::Builder::new_default();
    {
        // let mut basic = message::Builder::new_default();

        let mut complex = message.init_root::<proto_benchmarks::bench_capnp::complex::Builder>();
        // complex.set_basic(basic);
        complex.set_name("name");
        complex.set_reference("reference");

        {
            // let mut simple = basic.init_root::<proto_benchmarks::bench_capnp::basic::Builder>();
            let mut basic = complex.get_basic().unwrap();
            basic.set_id(12);
        }
    }
    c.bench_function("capnp_complex_write", |b| b.iter(|| complex_write(&message)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
