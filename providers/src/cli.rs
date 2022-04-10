use std::{
    future::Future,
    time::{Duration, Instant},
};
use yandex_cloud_api_core::auth::{RawTokenProvider, TokenInformation, TokenProviderError};

/// Yandex Cloud CLI-based IAM token provider
pub struct CliProvider {}

impl CliProvider {
    pub fn new() -> Self {
        CliProvider {}
    }

    fn get(&self) -> impl Future<Output = Result<TokenInformation, Error>> {
        async move {
            let mut cmd = tokio::process::Command::new("yc");
            cmd.arg("iam").arg("create-token");
            cmd.arg("-o").arg("json");
            let out = cmd.output().await?;
            if !out.status.success() {
                return Err(Error::Fail(
                    String::from_utf8_lossy(&out.stderr).into_owned(),
                ));
            }
            let resp: Response = serde_json::from_slice(&out.stdout)?;
            Ok(TokenInformation {
                token: resp.iam_token,
                expires: Some(Instant::now() + Duration::from_secs(9 * 3600 - 30)),
            })
        }
    }
}

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("io error")]
    Io(#[from] std::io::Error),
    #[error("failed to get token: {0}")]
    Fail(String),
    #[error("failed to parse yc output")]
    Serde(#[from] serde_json::Error),
}

#[derive(serde_derive::Deserialize)]
struct Response {
    iam_token: String,
    // TODO
    // expires_at: String
}

impl RawTokenProvider for CliProvider {
    fn get_token(
        &self,
    ) -> std::pin::Pin<Box<dyn Future<Output = Result<TokenInformation, TokenProviderError>> + Send>>
    {
        let fut = self.get();
        let fut = async move { fut.await.map_err(TokenProviderError::new) };
        Box::pin(fut)
    }
}
