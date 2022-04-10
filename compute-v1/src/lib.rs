pub mod pb;

use std::sync::Arc;

use tonic::transport::Channel;

use yandex_cloud_api_core::{auth::Iam, svc::AddToken};

use crate::pb::yandex::cloud::compute::v1::{
    instance_service_client::InstanceServiceClient,
    instancegroup::instance_group_service_client::InstanceGroupServiceClient,
};

/// Yandex Cloud Compute v1 client
pub struct ComputeV1 {
    iam: Arc<Iam>,
}

const ENDPOINT: &str = "compute.api.cloud.yandex.net";

impl ComputeV1 {
    pub async fn instances(
        &self,
    ) -> Result<
        InstanceServiceClient<tonic::codegen::InterceptedService<Channel, AddToken>>,
        tonic::transport::Error,
    > {
        let channel = Channel::from_static(ENDPOINT).connect().await?;
        let client =
            InstanceServiceClient::with_interceptor(channel, AddToken::new(self.iam.clone()));
        Ok(client)
    }

    pub async fn instance_groups(
        &self,
    ) -> Result<
        InstanceGroupServiceClient<tonic::codegen::InterceptedService<Channel, AddToken>>,
        tonic::transport::Error,
    > {
        let channel = Channel::from_static(ENDPOINT).connect().await?;
        let client =
            InstanceGroupServiceClient::with_interceptor(channel, AddToken::new(self.iam.clone()));
        Ok(client)
    }
}
