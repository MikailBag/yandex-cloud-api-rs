pub mod pb;

use std::sync::Arc;

use tonic::transport::Channel;

use yandex_cloud_api_core::{auth::Iam, svc::AddToken};

use crate::pb::yandex::cloud::kms::v1::{
    symmetric_crypto_service_client::SymmetricCryptoServiceClient,
    symmetric_key_service_client::SymmetricKeyServiceClient,
};

/// Yandex Cloud Key Management Service v1 client
pub struct KmsV1 {
    iam: Arc<Iam>,
}

const ENDPOINT: &str = "kms.api.cloud.yandex.net";

impl KmsV1 {
    pub async fn symmetric_crypto(
        &self,
    ) -> Result<
        SymmetricCryptoServiceClient<tonic::codegen::InterceptedService<Channel, AddToken>>,
        tonic::transport::Error,
    > {
        let channel = Channel::from_static(ENDPOINT).connect().await?;
        let client = SymmetricCryptoServiceClient::with_interceptor(
            channel,
            AddToken::new(self.iam.clone()),
        );
        Ok(client)
    }

    pub async fn symmetric_keys(
        &self,
    ) -> Result<
        SymmetricKeyServiceClient<tonic::codegen::InterceptedService<Channel, AddToken>>,
        tonic::transport::Error,
    > {
        let channel = Channel::from_static(ENDPOINT).connect().await?;
        let client =
            SymmetricKeyServiceClient::with_interceptor(channel, AddToken::new(self.iam.clone()));
        Ok(client)
    }
}
