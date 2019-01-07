extern crate protobuf_codegen_pure;

use capnpc::{RustEdition, CompilerCommand};

fn main() {
    protobuf_codegen_pure::run(protobuf_codegen_pure::Args {
        out_dir: "src",
        input: &["protos/bench.proto"],
        includes: &["protos"],
        customize: protobuf_codegen_pure::Customize {
            ..Default::default()
        },
    }).expect("protoc");

    CompilerCommand::new()
        .file("protos/bench.capnp")
        .edition(RustEdition::Rust2018)
        .run()
        .expect("compiling schema");
    
    // Convert protobuf .proto to FlatBuffers .fbs
    std::process::Command::new("flatc")
        .args(&["--proto", "-o", "protos", "protos/bench.proto"])
        .spawn()
        .expect("flatc");
    // Generate rust source
    std::process::Command::new("flatc")
        .args(&["--rust", "-o", "src", "protos/bench.fbs"])
        .spawn()
        .expect("flatc");
}