#[cfg(test)]
mod tests {
    use super::*;

    mod capnp {
        use capnp::{message, serialize, message::ReaderOptions};
        use super::*;

        mod basic {
            use super::*;

            #[test]
            fn it_deserializes() {
                let mut message = message::Builder::new_default();
                {
                    let mut simple = message.init_root::<bench_capnp::basic::Builder>();
                    simple.set_id(12);
                }
                let words = serialize::write_message_to_words(&message);

                let de = serialize::read_message_from_words(&words, ReaderOptions::new()).unwrap();
                let reader: bench_capnp::basic::Reader = de.get_root().unwrap();
                assert_eq!(reader.get_id(), 12);
            }
        }

        mod complex {
            use super::*;

            #[test]
            fn it_deserializes() {
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
                // assert_eq!(message.get_basic().unwrap().get_id().unwrap(), 12);
                let words = serialize::write_message_to_words(&message);
                let de = serialize::read_message_from_words(&words, ReaderOptions::new()).unwrap();
                let reader: bench_capnp::complex::Reader = de.get_root().unwrap();
                assert_eq!(reader.get_basic().unwrap().get_id(), 12);
                assert_eq!(reader.get_name().unwrap(), "name");
            }
        }
    }

    mod protobuf {
        use protobuf::{Message, parse_from_bytes};
        use super::*;

        mod basic {
            use super::*;

            #[test]
            fn it_deserializes() {
                let mut basic = bench::Basic::new();
                basic.set_id(12);
                let bytes = basic.write_to_bytes().unwrap();

                let parsed = parse_from_bytes::<bench::Basic>(&bytes).unwrap();
                assert_eq!(parsed.get_id(), 12);
            }
        }

        mod complex {
            use super::*;

            #[test]
            fn it_deserializes() {
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
    }

    mod prost {
        mod basic {
            use prost::Message;
            use bench_prost::Basic;

            #[test]
            fn it_deserializes() {
                let basic = Basic { id: 12 };

                let mut bytes = Vec::new();
                basic.encode(&mut bytes).unwrap();

                let parsed = Basic::decode(&bytes).unwrap();
                assert_eq!(parsed.id, 12);
            }
        }

        mod complex {
            use prost::Message;
            use bench_prost::{Basic, Complex};

            #[test]
            fn it_deserializes() {
                let basic = Basic {
                    id: 12,
                };
                let stat = Complex {
                    basic,
                    name: "name".into(),
                    reference: "reference".into(),
                };

                let mut bytes = Vec::new();
                stat.encode(&mut bytes).unwrap();

                let parsed = Complex::decode(&bytes).unwrap();
                assert_eq!(parsed.basic.id, 12);
                assert_eq!(parsed.name, "name");
            }
        }
    }
}

extern crate capnp;
extern crate prost;
extern crate protobuf;
extern crate bytes;

#[macro_use]
extern crate prost_derive;

pub mod bench;
pub mod bench_capnp;

pub mod bench_prost {
    include!(concat!(env!("OUT_DIR"), "/bench.rs"));
}

pub use bench as bench_protobuf;
// pub struct Basic {
//     id: u64
// }

// pub struct Complex<'a> {
//     name: String,
//     basic: Basic,
//     reference: &'a str,
// }

