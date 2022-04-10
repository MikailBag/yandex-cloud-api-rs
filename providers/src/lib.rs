use yandex_cloud_api_core::auth::{Iam, TokenProviderError};

pub mod auth_key;
pub mod imds;
pub mod cli;

pub async fn init_inside_cloud() -> Result<Iam, TokenProviderError> {
    Iam::new(imds::ImdsProvider::new()).await
}
