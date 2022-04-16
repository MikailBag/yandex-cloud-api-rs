use tonic::{codegen::InterceptedService, transport::Channel, transport::Error};

use crate::pb::yandex::cloud::compute::v1::{
    instance_service_client::InstanceServiceClient,
    instancegroup::instance_group_service_client::InstanceGroupServiceClient,
};
use yandex_cloud_api_core::{auth::Iam, svc::AddToken};

/// Yandex Cloud compute v1 client
pub struct ComputeV1 {
    iam: Iam,
}

const ENDPOINT: &str = "compute.api.cloud.yandex.net";

impl ComputeV1 {
    pub fn new(iam: Iam) -> Self {
        Self { iam }
    }
    pub async fn instances(
        &self,
    ) -> Result<InstanceServiceClient<InterceptedService<Channel, AddToken>>, Error> {
        let channel = Channel::from_static(ENDPOINT).connect().await?;
        let client =
            InstanceServiceClient::with_interceptor(channel, AddToken::new(self.iam.clone()));
        Ok(client)
    }
    pub async fn instance_groups(
        &self,
    ) -> Result<InstanceGroupServiceClient<InterceptedService<Channel, AddToken>>, Error> {
        let channel = Channel::from_static(ENDPOINT).connect().await?;
        let client =
            InstanceGroupServiceClient::with_interceptor(channel, AddToken::new(self.iam.clone()));
        Ok(client)
    }
}
