#[cfg(test)]
mod tests {
    use super::*;

    mod capnp_test {
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

    mod protobuf_test {
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

    mod flatbuffers_test {
        use flatbuffers::*;

        mod basic {
            use super::*;
            use crate::bench_generated as bench_fbs;

            #[test]
            fn it_deserializes() {
                const ID: u64 = 12;
                let mut builder = FlatBufferBuilder::new();
                let basic_args = bench_fbs::bench::BasicArgs { id: ID, .. Default::default() };
                let basic: WIPOffset<_> = bench_fbs::bench::Basic::create(&mut builder, &basic_args);
                builder.finish_minimal(basic);
                //let parsed = bench_generated::bench::Basic::follow(builder.finished_data(), 0);
                let parsed = flatbuffers::get_root::<bench_fbs::bench::Basic>(builder.finished_data());
                assert_eq!(parsed.id(), ID);
            }
        }

        mod complex {
            use super::*;
            use crate::bench_generated as bench_fbs;

            #[test]
            fn it_deserializes() {
                const ID: u64 = 12;
                const NAME: &str = "name";
                const REFERENCE: &str = "reference";
                let mut builder = flatbuffers::FlatBufferBuilder::new();
                {
                    let args = bench_fbs::bench::BasicArgs{id: ID};
                    let basic = Some(bench_fbs::bench::Basic::create(&mut builder, &args));
                    let name = Some(builder.create_string(NAME));
                    let reference = Some(builder.create_string(REFERENCE));
                    let args = bench_fbs::bench::ComplexArgs{ basic, name, reference };
                    let complex = bench_fbs::bench::Complex::create(&mut builder, &args);
                    builder.finish_minimal(complex);
                }
                let parsed = flatbuffers::get_root::<bench_fbs::bench::Complex>(builder.finished_data());
                assert_eq!(parsed.basic().id(), ID);
                assert_eq!(parsed.name(), NAME);
                assert_eq!(parsed.reference(), REFERENCE);
            }
        }
    }
}

pub mod bench;
pub mod bench_capnp;
pub mod bench_generated;
