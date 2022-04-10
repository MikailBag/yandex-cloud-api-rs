use std::{future::Future, time::Instant};

use yandex_cloud_api_core::auth::{RawTokenProvider, TokenInformation, TokenProviderError};

/// Instance metadata service-based IAM token provider
pub struct ImdsProvider {
    client: reqwest::Client,
}

impl ImdsProvider {
    pub fn new() -> Self {
        ImdsProvider {
            client: reqwest::Client::new(),
        }
    }

    fn get_token(&self) -> impl Future<Output = Result<TokenInformation, Error>> + 'static {
        let url =
            "http://169.254.169.254/computeMetadata/v1/instance/service-accounts/default/token";
        let client = self.client.clone();
        async move {
            let response = client
                .get(url)
                .header("Metadata-Flavor", "Google")
                .send()
                .await?;
            response.error_for_status_ref()?;

            let body: Response = response.json().await?;
            Ok(TokenInformation {
                token: body.access_token,
                expires: Some(Instant::now() + std::time::Duration::from_secs(body.expires_in)),
            })
        }
    }
}

#[derive(serde_derive::Deserialize)]
struct Response {
    access_token: String,
    expires_in: u64,
}

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("http error")]
    Reqwest(#[from] reqwest::Error),
}

impl RawTokenProvider for ImdsProvider {
    fn get_token(
        &self,
    ) -> std::pin::Pin<
        Box<dyn std::future::Future<Output = Result<TokenInformation, TokenProviderError>> + Send>,
    > {
        let fut = self.get_token();

        let fut = async move { fut.await.map_err(TokenProviderError::new) };
        Box::pin(fut)
    }
}
