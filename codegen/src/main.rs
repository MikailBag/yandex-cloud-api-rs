fn make_builder(service: &str, version: &str) -> tonic_build::Builder {
    tonic_build::configure()
        .build_client(true)
        .build_server(false)
        .out_dir(format!("./{service}-{version}/src/gen"))
}

fn main() {
    make_builder("compute", "v1")
        .compile(
            &[
                "./vendor/yandex/cloud/compute/v1/instance_service.proto",
                "./vendor/yandex/cloud/compute/v1/instancegroup/instance_group_service.proto",
            ],
            &["./vendor"],
        )
        .unwrap();

    make_builder("kms", "v1")
        .compile(
            &["./vendor/yandex/cloud/kms/v1/symmetric_crypto_service.proto"],
            &["./vendor"],
        )
        .unwrap();
}
