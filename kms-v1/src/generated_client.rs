use tonic::{codegen::InterceptedService, transport::Channel, transport::Error};

use crate::pb::yandex::cloud::kms::v1::{
    symmetric_crypto_service_client::SymmetricCryptoServiceClient,
    symmetric_key_service_client::SymmetricKeyServiceClient,
};
use yandex_cloud_api_core::{auth::Iam, svc::AddToken};

/// Yandex Cloud kms v1 client
pub struct KmsV1 {
    iam: Iam,
}

const ENDPOINT: &str = "https://kms.api.cloud.yandex.net";

impl KmsV1 {
    pub fn new(iam: Iam) -> Self {
        Self { iam }
    }
    pub async fn symmetric_crypto(
        &self,
    ) -> Result<SymmetricCryptoServiceClient<InterceptedService<Channel, AddToken>>, Error> {
        let channel = Channel::from_static(ENDPOINT).connect().await?;
        let client = SymmetricCryptoServiceClient::with_interceptor(
            channel,
            AddToken::new(self.iam.clone()),
        );
        Ok(client)
    }
    pub async fn symmetric_keys(
        &self,
    ) -> Result<SymmetricKeyServiceClient<InterceptedService<Channel, AddToken>>, Error> {
        let channel = Channel::from_static(ENDPOINT).connect().await?;
        let client =
            SymmetricKeyServiceClient::with_interceptor(channel, AddToken::new(self.iam.clone()));
        Ok(client)
    }
}
