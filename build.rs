#[cfg(feature = "protobuf")]
extern crate protobuf_codegen_pure;

#[cfg(feature = "protobuf")]
fn compile_protobufs() {
    // Generates Rust code from protocol buffer definitions
    protobuf_codegen_pure::run(protobuf_codegen_pure::Args {
        out_dir: "src/protos",
        input: &["./schema/channel_message.proto"],
        includes: &["./schema"],
        customize: protobuf_codegen_pure::Customize {
            ..Default::default()
        },
    }).expect("Protobuf codegen error");
}

pub fn main() {
    #[cfg(feature = "protobuf")]
    compile_protobufs();
}
