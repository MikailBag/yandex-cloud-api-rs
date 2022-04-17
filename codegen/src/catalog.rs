use crate::service::CloudService;

fn make_compute_v1() -> CloudService {
    CloudService::new("compute", "v1")
        .with_endpoint("compute.api.cloud.yandex.net")
        .with_client_name("ComputeV1")
        .with_grpc(
            "instances",
            "instance_service.proto",
            "instance_service_client::InstanceServiceClient",
        )
        .with_grpc(
            "instance_groups",
            "instancegroup/instance_group_service.proto",
            "instancegroup::instance_group_service_client::InstanceGroupServiceClient",
        )
}

fn make_kms_v1() -> CloudService {
    CloudService::new("kms", "v1")
        .with_endpoint("kms.api.cloud.yandex.net")
        .with_client_name("KmsV1")
        .with_grpc(
            "symmetric_crypto",
            "symmetric_crypto_service.proto",
            "symmetric_crypto_service_client::SymmetricCryptoServiceClient",
        )
        .with_grpc(
            "symmetric_keys",
            "symmetric_key_service.proto",
            "symmetric_key_service_client::SymmetricKeyServiceClient",
        )
}

fn make_ydb_v1() -> CloudService {
    CloudService::new("ydb", "v1")
        .with_endpoint("ydb.api.cloud.yandex.net")
        .with_client_name("YdbV1")
        .with_grpc(
            "backups",
            "backup_service.proto",
            "backup_service_client::BackupServiceClient",
        )
        .with_grpc(
            "databases",
            "database_service.proto",
            "database_service_client::DatabaseServiceClient",
        )
        .with_grpc(
            "locations",
            "location_service.proto",
            "location_service_client::LocationServiceClient",
        )
}

pub fn services() -> Vec<CloudService> {
    vec![make_compute_v1(), make_kms_v1(), make_ydb_v1()]
}
