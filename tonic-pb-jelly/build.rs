use std::env;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    // first pass: generate descriptor (for grpc reflection)
    tonic_build::configure()
        .file_descriptor_set_path(out_dir.join("helloworld_descriptor.bin"))
        .compile_protos(
            &["../proto/helloworld.proto"],
            &["../proto", "../proto_include"],
        )
        .unwrap();

    // second pass: generate server && client code using generated code from pb-jelly && custom codec 
    let greeter_service = tonic_build::manual::Service::builder()
        .name("Greeter")
        .package("helloworld")
        .method(
            tonic_build::manual::Method::builder()
                .name("say_hello")
                .route_name("SayHello")
                .input_type("proto_helloworld::proto_helloworld::HelloRequest")
                .output_type("proto_helloworld::proto_helloworld::HelloReply")
                .codec_path("crate::codec::JellyCodec")
                .build(),
        )
        .build();

    tonic_build::manual::Builder::new()
        .compile(&[greeter_service]);

    Ok(())
}
