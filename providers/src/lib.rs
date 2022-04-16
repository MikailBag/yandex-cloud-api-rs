use yandex_cloud_api_core::auth::{Iam, TokenProviderError};

pub mod auth_key;
pub mod cli;
pub mod imds;

pub async fn init_inside_cloud() -> Result<Iam, TokenProviderError> {
    Iam::new(imds::ImdsProvider::new()).await
}

pub async fn init_cli() -> Result<Iam, TokenProviderError> {
    Iam::new(cli::CliProvider::new()).await
}

#[derive(Debug, thiserror::Error)]
#[error("no token providers were found")]
pub struct TokenProvidersNotFound {
    _priv: (),
}

pub async fn init() -> Result<Iam, TokenProviderError> {
    if let Ok(iam) = init_inside_cloud().await {
        return Ok(iam);
    }
    if let Ok(iam) = init_cli().await {
        return Ok(iam);
    }
    Err(TokenProviderError::new(TokenProvidersNotFound {
        _priv: (),
    }))
}
