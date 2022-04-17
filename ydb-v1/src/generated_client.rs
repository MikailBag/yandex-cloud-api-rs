use tonic::{codegen::InterceptedService, transport::Channel, transport::Error};

use crate::pb::yandex::cloud::ydb::v1::{
    backup_service_client::BackupServiceClient, database_service_client::DatabaseServiceClient,
    location_service_client::LocationServiceClient,
};
use yandex_cloud_api_core::{auth::Iam, svc::AddToken};

/// Yandex Cloud ydb v1 client
pub struct YdbV1 {
    iam: Iam,
}

const ENDPOINT: &str = "ydb.api.cloud.yandex.net";

impl YdbV1 {
    pub fn new(iam: Iam) -> Self {
        Self { iam }
    }
    pub async fn backups(
        &self,
    ) -> Result<BackupServiceClient<InterceptedService<Channel, AddToken>>, Error> {
        let channel = Channel::from_static(ENDPOINT).connect().await?;
        let client =
            BackupServiceClient::with_interceptor(channel, AddToken::new(self.iam.clone()));
        Ok(client)
    }
    pub async fn databases(
        &self,
    ) -> Result<DatabaseServiceClient<InterceptedService<Channel, AddToken>>, Error> {
        let channel = Channel::from_static(ENDPOINT).connect().await?;
        let client =
            DatabaseServiceClient::with_interceptor(channel, AddToken::new(self.iam.clone()));
        Ok(client)
    }
    pub async fn locations(
        &self,
    ) -> Result<LocationServiceClient<InterceptedService<Channel, AddToken>>, Error> {
        let channel = Channel::from_static(ENDPOINT).connect().await?;
        let client =
            LocationServiceClient::with_interceptor(channel, AddToken::new(self.iam.clone()));
        Ok(client)
    }
}
