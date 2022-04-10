use tonic::metadata::MetadataValue;

use crate::auth::{Iam, TokenProviderError};
use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll}, sync::Arc,
};

/// Interceptor
pub struct AddToken {
    iam: Arc<Iam>,
}

impl AddToken {
    pub fn new(iam: Arc<Iam>) -> Self {
        AddToken { iam }
    }
}

impl tonic::service::Interceptor for AddToken {
    fn call(&mut self, mut request: tonic::Request<()>) -> Result<tonic::Request<()>, tonic::Status> {
        let token = match self.iam.get() {
            Ok(t) => t,
            Err(err) => {
                return Err(tonic::Status::unavailable(format!("{err}")))
            }
        };
        let token = match MetadataValue::from_str(token.get()) {
            Ok(mv) => mv,
            Err(err) => return Err(tonic::Status::internal(format!("invalid IAM token: {err}")))
        };
        request.metadata_mut().insert("authorization", token);
        Ok(request)
    }
}