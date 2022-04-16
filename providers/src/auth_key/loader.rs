use std::path::PathBuf;

use crate::auth_key::{AuthorizedKeyLocation, AuthorizedKeySearchSpec, Settings};

fn take_string(val: &mut serde_json::Value, pathes: &[&str]) -> Option<String> {
    for path in pathes {
        let v = val.pointer_mut(path).take();
        let v = match v {
            Some(v) if v.is_string() => v.take(),
            _ => continue,
        };
        if let serde_json::Value::String(s) = v {
            return Some(s);
        } else {
            unreachable!()
        }
    }
    None
}

pub(crate) fn make_settings_from_json(json: &mut serde_json::Value) -> Option<Settings> {
    let private_key = take_string(json, &["/privateKey", "/private_key"])?;
    let kid = take_string(json, &["/key/id", "/id"])?;
    let sid = take_string(json, &["/key/serviceAccountId", "/service_account_id"])?;
    Some(Settings {
        key_id: kid.to_string(),
        sa_id: sid.to_string(),
        pem: private_key.to_string(),
    })
}

async fn load_key_location(loc: &AuthorizedKeyLocation) -> Option<Vec<u8>> {
    match loc {
        AuthorizedKeyLocation::File(path) => tokio::fs::read(path).await.ok(),
        AuthorizedKeyLocation::PathInEnvVar(var) => {
            let path: PathBuf = std::env::var_os(var)?.into();
            tokio::fs::read(path).await.ok()
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum LoadError {
    #[error("authorized key not found")]
    NotFound,

    #[error("authorized key is invalid json")]
    ParseError(#[from] serde_json::error::Error),
    #[error("authorized key has incorrect format")]
    IncorrectFormat,
}

pub(crate) async fn load_from_search_spec(
    spec: &AuthorizedKeySearchSpec,
) -> Result<Settings, LoadError> {
    for location in &spec.locations {
        let key = match load_key_location(location).await {
            Some(k) => k,
            None => continue,
        };
        let key: serde_json::Value = serde_json::from_slice(&key)?;
        let settings = Settings::from_json(&mut { key }).ok_or(LoadError::IncorrectFormat)?;
        return Ok(settings);
    }
    return Err(LoadError::NotFound);
}
