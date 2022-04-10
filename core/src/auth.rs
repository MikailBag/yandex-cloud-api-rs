use std::{
    error::Error,
    fmt::Formatter,
    fmt::{Debug, Display},
    future::Future,
    pin::Pin,
    sync::{Arc, Mutex, Weak},
    time::{Duration, Instant},
};

/// Opaque error
pub struct TokenProviderError(Box<dyn Error + Send + Sync + 'static>);

impl TokenProviderError {
    pub fn new<E: Error + Send + Sync + 'static>(err: E) -> Self {
        TokenProviderError(Box::new(err))
    }
}

/// Represents IAM token provider. It can be used directly or
/// to create service clients
#[derive(Clone)]
pub struct Iam {
    provider: Arc<dyn RawTokenProvider>,
    state: Arc<Mutex<State>>,
    settings: Arc<Settings>,
}

/// Additional [`Iam`] settings.
#[non_exhaustive]
pub struct Settings {
    /// How often token should be refreshed.
    /// Default value is 1 hour.
    pub refresh_after: Duration,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            refresh_after: Duration::from_secs(3600),
        }
    }
}

/// Represents IAM token.
#[derive(Clone)]
pub struct IamToken {
    token: String,
    state_ref: Weak<Mutex<State>>,
}

impl IamToken {
    /// Returns actual token
    pub fn get(&self) -> &str {
        &self.token
    }

    /// Signals to the `Iam` instance which created this token,
    /// that it is no longer valid (e.g. expired) and should be replaced
    pub fn invalidate(&self) {
        let ptr = match self.state_ref.upgrade() {
            Some(p) => p,
            None => return,
        };
        let mut g = ptr.lock().unwrap();
        g.current_token = None;
    }
}

impl Debug for IamToken {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("IamToken")
            .field("token", &"[omitted]")
            .finish_non_exhaustive()
    }
}

impl Iam {
    pub async fn new<P: RawTokenProvider>(provider: P) -> Result<Self, TokenProviderError> {
        let this = Iam::with_settings(provider, Settings::default());
        this.refresh().await?;
        Ok(this)
    }

    pub fn with_settings<P: RawTokenProvider>(provider: P, settings: Settings) -> Self {
        let state = State {
            current_token: None,
        };
        Iam {
            provider: Arc::new(provider),
            state: Arc::new(Mutex::new(state)),
            settings: Arc::new(settings),
        }
    }

    fn make_wrapper(&self, token: String) -> IamToken {
        IamToken {
            token,
            state_ref: Arc::downgrade(&self.state),
        }
    }

    fn get_cached(&self) -> Option<String> {
        let state = self.state.lock().unwrap();
        let info = state.current_token.as_ref()?;
        Some(info.token.clone())
    }

    /// Returns reference to underlying provider
    pub fn inner(&self) -> &dyn RawTokenProvider {
        &*self.provider
    }

    /// Refreshed cached token. Generally you only need to call it once
    /// after [`Iam::with_settings`]
    pub async fn refresh(&self) -> Result<IamToken, TokenProviderError> {
        let info = self.provider.get_token().await?;
        let token = self.make_wrapper(info.token.clone());
        {
            self.state.lock().unwrap().current_token = Some(info);
        }
        Ok(token)
    }

    /// Returns IAM token. This function either uses cached token or queries
    /// provider.
    pub fn get(&self) -> Result<IamToken, TokenProviderError> {
        if let Some(t) = self.get_cached() {
            return Ok(self.make_wrapper(t));
        }
        
        Err(TokenProviderError::new(Uninitialized))
    }
}

struct State {
    current_token: Option<TokenInformation>,
}

#[derive(Debug)]
struct Uninitialized;

impl std::fmt::Display for Uninitialized {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt("no tokens fetched yet", f)
    }
}

impl std::error::Error for Uninitialized {}

pub struct TokenInformation {
    /// IAM token itself
    pub token: String,
    /// Expiration date
    pub expires: Option<Instant>,
}

impl Debug for TokenProviderError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        Debug::fmt(&*self.0, f)
    }
}
impl Display for TokenProviderError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        Display::fmt(&*self.0, f)
    }
}

impl Error for TokenProviderError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.0.source()
    }
}

pub trait RawTokenProvider: Send + Sync + 'static {
    fn get_token(
        &self,
    ) -> Pin<Box<dyn Future<Output = Result<TokenInformation, TokenProviderError>> + Send>>;
}
