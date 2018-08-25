extern crate pb_rs;
extern crate capnpc;
extern crate protobuf_codegen_pure;

use std::path::PathBuf;
use pb_rs::types::{Config, FileDescriptor};

fn main() {
    protobuf_codegen_pure::run(protobuf_codegen_pure::Args {
        out_dir: "src",
        input: &["protos/bench.proto"],
        includes: &["protos"],
        customize: protobuf_codegen_pure::Customize {
            ..Default::default()
        },
    }).expect("protoc");

    ::capnpc::CompilerCommand::new()
        .file("protos/bench.capnp")
        .run()
        .expect("compiling schema");
    
    let config = Config {
        in_file: PathBuf::from("protos/bench.proto"),
        out_file: PathBuf::from("src/bench_quick.rs"),
        single_module: true,
        import_search_path: vec![PathBuf::from("protos")],
        no_output: false,
    };

    FileDescriptor::write_proto(&config).unwrap();
}
