use yandex_cloud_api_core::auth::{Iam, TokenProviderError};

pub mod auth_key;
pub mod cli;
pub mod imds;

pub async fn init_inside_cloud() -> Result<Iam, TokenProviderError> {
    Iam::new(imds::ImdsProvider::new()).await
}
