#[cfg(test)]
mod tests {
    use capnp::{message, serialize, message::ReaderOptions};
    use protobuf::{Message, parse_from_bytes};

    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn it_deserializes_complex_capnp() {
        let mut message = message::Builder::new_default();
        {
            // let mut basic = message::Builder::new_default();

            let mut complex = message.init_root::<bench_capnp::complex::Builder>();
            // complex.set_basic(basic);
            complex.set_name("name");
            complex.set_reference("reference");

            {
                // let mut simple = basic.init_root::<proto_benchmarks::bench_capnp::basic::Builder>();
                let mut basic = complex.get_basic().unwrap();
                basic.set_id(12);
            }
        }
        // assert_eq!(message.get_basic().unwrap().get_id().unwrap(), 12);
        let words = serialize::write_message_to_words(&message);
        let de = serialize::read_message_from_words(&words, ReaderOptions::new()).unwrap();
        let reader: bench_capnp::complex::Reader = de.get_root().unwrap();
        assert_eq!(reader.get_basic().unwrap().get_id(), 12);
        assert_eq!(reader.get_name().unwrap(), "name");
    }

    #[test]
    fn it_deserializes_complex_protobuf() {
        let mut basic = bench::Basic::new();
        basic.set_id(12);
        let mut stat = bench::Complex::new();
        stat.set_basic(basic);
        stat.set_name("name".into());
        stat.set_reference("reference".into());
        let bytes = stat.write_to_bytes().unwrap();

        let parsed = parse_from_bytes::<bench::Complex>(&bytes).unwrap();
        assert_eq!(parsed.get_basic().get_id(), 12);
        assert_eq!(parsed.get_name(), "name");
    }
}

extern crate capnp;
extern crate protobuf;

pub mod bench;
pub mod bench_capnp;

pub use bench as bench_protobuf;
// pub struct Basic {
//     id: u64
// }

// pub struct Complex<'a> {
//     name: String,
//     basic: Basic,
//     reference: &'a str,
// }

