use tonic_prost_build::configure;

fn main() {
    configure()
        .build_server(true)
        .build_client(true)
        .compile_protos(
            &["proto/service.proto"],
            &["proto"],
        )
        .unwrap();
}