
extern crate capnpc;
extern crate protobuf_codegen_pure;

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
}