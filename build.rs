extern crate protoc_rust;

fn main() {
    protoc_rust::run(protoc_rust::Args {
        out_dir: "src/",
        input: &["src/protos/matcha_pb.proto"],
        includes: &[],
    }).expect("protoc");
}
