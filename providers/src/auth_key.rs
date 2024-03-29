mod loader;

use std::{
    future::Future,
    path::PathBuf,
    time::{Duration, Instant},
};

use yandex_cloud_api_core::auth::{RawTokenProvider, TokenInformation, TokenProviderError};

/// Service account authorized key-based IAM token provider
pub struct AuthorizedKeyProvider {
    client: reqwest::Client,
    key: jsonwebtoken::EncodingKey,
    header: jsonwebtoken::Header,
    claims: Claims,
}

/// Settings used for provider setup
pub struct Settings {
    /// Authorized key id
    pub key_id: String,
    /// Service account id
    pub sa_id: String,
    /// Private key in PEM format
    pub pem: String,
}
pub enum AuthorizedKeyLocation {
    File(PathBuf),
    PathInEnvVar(String),
}

pub struct AuthorizedKeySearchSpec {
    pub locations: Vec<AuthorizedKeyLocation>,
}

#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub struct SearchError(loader::LoadError);

impl Settings {
    /// Creates settings from CLI-style or API-style JSON
    pub fn from_json(json: &mut serde_json::Value) -> Option<Self> {
        loader::make_settings_from_json(json)
    }

    /// Searches for key in specified location
    pub async fn search(spec: &AuthorizedKeySearchSpec) -> Result<Self, SearchError> {
        loader::load_from_search_spec(spec)
            .await
            .map_err(SearchError)
    }
}

#[derive(Clone, serde_derive::Serialize)]
struct Claims {
    iss: String,
    aud: &'static str,
    iat: u64,
    exp: u64,
}

#[derive(thiserror::Error, Debug)]
pub enum InitError {
    #[error("invalid private key")]
    InvalidKey(#[from] jsonwebtoken::errors::Error),
}

#[derive(thiserror::Error, Debug)]
enum Error {
    #[error("failed to generate JWT")]
    Jwt(#[from] jsonwebtoken::errors::Error),
    #[error("request error")]
    Http(#[from] reqwest::Error),
}

#[derive(serde_derive::Deserialize)]
struct Response {
    #[serde(rename = "iamToken")]
    token: String,
    #[serde(rename = "expiresIn")]
    expires: u64,
}

impl AuthorizedKeyProvider {
    pub fn new(settings: Settings) -> Result<Self, InitError> {
        let mut header = jsonwebtoken::Header::new(jsonwebtoken::Algorithm::PS256);

        header.kid = Some(settings.key_id);

        let claims = Claims {
            iss: settings.sa_id,
            aud: "https://iam.api.cloud.yandex.net/iam/v1/tokens",
            iat: 0,
            exp: 0,
        };

        let p = AuthorizedKeyProvider {
            client: reqwest::Client::new(),
            key: jsonwebtoken::EncodingKey::from_rsa_pem(&settings.pem.as_bytes())?,
            header,
            claims,
        };
        Ok(p)
    }

    fn make_jwt(&self) -> Result<String, Error> {
        let now = jsonwebtoken::get_current_timestamp();
        let expires = now + 3600;
        let mut claims = self.claims.clone();
        claims.iat = now;
        claims.exp = expires;

        let token = jsonwebtoken::encode(&self.header, &claims, &self.key)?;
        Ok(token)
    }

    fn get_token(&self) -> impl Future<Output = Result<TokenInformation, Error>> + Send + 'static {
        let client = self.client.clone();
        let jwt = self.make_jwt();
        async move {
            let jwt = jwt?;

            let req_body = serde_json::json!({ "jwt": jwt });
            let response = client
                .post("https://iam.api.cloud.yandex.net/iam/v1/tokens")
                .json(&req_body)
                .send()
                .await?;
            response.error_for_status_ref()?;
            let body: Response = response.json().await?;
            Ok(TokenInformation {
                token: body.token,
                expires: Some(Instant::now() + Duration::from_secs(body.expires)),
            })
        }
    }
}

impl RawTokenProvider for AuthorizedKeyProvider {
    fn get_token(
        &self,
    ) -> std::pin::Pin<Box<dyn Future<Output = Result<TokenInformation, TokenProviderError>> + Send>>
    {
        let fut = self.get_token();

        let fut = async move { fut.await.map_err(TokenProviderError::new) };
        Box::pin(fut)
    }
}
