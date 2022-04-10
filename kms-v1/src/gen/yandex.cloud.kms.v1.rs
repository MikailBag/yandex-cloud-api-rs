/// A symmetric KMS key that may contain several versions of the cryptographic material.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SymmetricKey {
    /// ID of the key.
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    /// ID of the folder that the key belongs to.
    #[prost(string, tag="2")]
    pub folder_id: ::prost::alloc::string::String,
    /// Time when the key was created.
    #[prost(message, optional, tag="3")]
    pub created_at: ::core::option::Option<::prost_types::Timestamp>,
    /// Name of the key.
    #[prost(string, tag="4")]
    pub name: ::prost::alloc::string::String,
    /// Description of the key.
    #[prost(string, tag="5")]
    pub description: ::prost::alloc::string::String,
    /// Custom labels for the key as `key:value` pairs. Maximum 64 per key.
    #[prost(map="string, string", tag="6")]
    pub labels: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Current status of the key.
    #[prost(enumeration="symmetric_key::Status", tag="7")]
    pub status: i32,
    /// Primary version of the key, used as the default for all encrypt/decrypt operations,
    /// when no version ID is specified.
    #[prost(message, optional, tag="8")]
    pub primary_version: ::core::option::Option<SymmetricKeyVersion>,
    /// Default encryption algorithm to be used with new versions of the key.
    #[prost(enumeration="SymmetricAlgorithm", tag="9")]
    pub default_algorithm: i32,
    /// Time of the last key rotation (time when the last version was created).
    /// Empty if the key does not have versions yet.
    #[prost(message, optional, tag="10")]
    pub rotated_at: ::core::option::Option<::prost_types::Timestamp>,
    /// Time period between automatic key rotations.
    #[prost(message, optional, tag="11")]
    pub rotation_period: ::core::option::Option<::prost_types::Duration>,
    /// Flag that inhibits deletion of the key
    #[prost(bool, tag="12")]
    pub deletion_protection: bool,
}
/// Nested message and enum types in `SymmetricKey`.
pub mod symmetric_key {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Status {
        Unspecified = 0,
        /// The key is being created.
        Creating = 1,
        /// The key is active and can be used for encryption and decryption.
        /// Can be set to INACTIVE using the \[SymmetricKeyService.Update\] method.
        Active = 2,
        /// The key is inactive and unusable.
        /// Can be set to ACTIVE using the \[SymmetricKeyService.Update\] method.
        Inactive = 3,
    }
}
/// Symmetric KMS key version: metadata about actual cryptographic data.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SymmetricKeyVersion {
    /// ID of the key version.
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    /// ID of the symmetric KMS key that the version belongs to.
    #[prost(string, tag="2")]
    pub key_id: ::prost::alloc::string::String,
    /// Status of the key version.
    #[prost(enumeration="symmetric_key_version::Status", tag="3")]
    pub status: i32,
    /// Encryption algorithm that should be used when using the key version to encrypt plaintext.
    #[prost(enumeration="SymmetricAlgorithm", tag="4")]
    pub algorithm: i32,
    /// Time when the key version was created.
    #[prost(message, optional, tag="5")]
    pub created_at: ::core::option::Option<::prost_types::Timestamp>,
    /// Indication of a primary version, that is to be used by default for all cryptographic
    /// operations that don't have a key version explicitly specified.
    #[prost(bool, tag="6")]
    pub primary: bool,
    /// Time when the key version is going to be destroyed. Empty unless the status
    /// is `SCHEDULED_FOR_DESTRUCTION`.
    #[prost(message, optional, tag="7")]
    pub destroy_at: ::core::option::Option<::prost_types::Timestamp>,
    /// Indication of the version that is hosted by HSM.
    #[prost(bool, tag="8")]
    pub hosted_by_hsm: bool,
}
/// Nested message and enum types in `SymmetricKeyVersion`.
pub mod symmetric_key_version {
    /// Possible version status.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Status {
        Unspecified = 0,
        /// The version is active and can be used for encryption and decryption.
        Active = 1,
        /// The version is scheduled for destruction, the time when it will be destroyed
        /// is specified in the \[SymmetricKeyVersion.destroy_at\] field.
        ScheduledForDestruction = 2,
        /// The version is destroyed and cannot be recovered.
        Destroyed = 3,
    }
}
/// Supported symmetric encryption algorithms.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SymmetricAlgorithm {
    Unspecified = 0,
    /// AES algorithm with 128-bit keys.
    Aes128 = 1,
    /// AES algorithm with 192-bit keys.
    Aes192 = 2,
    /// AES algorithm with 256-bit keys.
    Aes256 = 3,
    /// AES algorithm with 256-bit keys hosted by HSM
    Aes256Hsm = 4,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SymmetricEncryptRequest {
    /// ID of the symmetric KMS key to use for encryption.
    #[prost(string, tag="1")]
    pub key_id: ::prost::alloc::string::String,
    /// ID of the key version to encrypt plaintext with.
    /// Defaults to the primary version if not specified.
    #[prost(string, tag="2")]
    pub version_id: ::prost::alloc::string::String,
    /// Additional authenticated data (AAD context), optional.
    /// If specified, this data will be required for decryption with the \[SymmetricDecryptRequest\].
    /// Should be encoded with base64.
    #[prost(bytes="vec", tag="3")]
    pub aad_context: ::prost::alloc::vec::Vec<u8>,
    /// Plaintext to be encrypted.
    /// Should be encoded with base64.
    #[prost(bytes="vec", tag="4")]
    pub plaintext: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SymmetricEncryptResponse {
    /// ID of the symmetric KMS key that was used for encryption.
    #[prost(string, tag="1")]
    pub key_id: ::prost::alloc::string::String,
    /// ID of the key version that was used for encryption.
    #[prost(string, tag="2")]
    pub version_id: ::prost::alloc::string::String,
    /// Resulting ciphertext.
    #[prost(bytes="vec", tag="3")]
    pub ciphertext: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SymmetricDecryptRequest {
    /// ID of the symmetric KMS key to use for decryption.
    #[prost(string, tag="1")]
    pub key_id: ::prost::alloc::string::String,
    /// Additional authenticated data, must be the same as was provided
    /// in the corresponding \[SymmetricEncryptRequest\].
    /// Should be encoded with base64.
    #[prost(bytes="vec", tag="2")]
    pub aad_context: ::prost::alloc::vec::Vec<u8>,
    /// Ciphertext to be decrypted.
    /// Should be encoded with base64.
    #[prost(bytes="vec", tag="3")]
    pub ciphertext: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SymmetricDecryptResponse {
    /// ID of the symmetric KMS key that was used for decryption.
    #[prost(string, tag="1")]
    pub key_id: ::prost::alloc::string::String,
    /// ID of the key version that was used for decryption.
    #[prost(string, tag="2")]
    pub version_id: ::prost::alloc::string::String,
    /// Decrypted plaintext.
    #[prost(bytes="vec", tag="3")]
    pub plaintext: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateDataKeyRequest {
    /// ID of the symmetric KMS key that the generated data key should be encrypted with.
    #[prost(string, tag="1")]
    pub key_id: ::prost::alloc::string::String,
    /// ID of the key version to encrypt the generated data key with.
    /// Defaults to the primary version if not specified.
    #[prost(string, tag="2")]
    pub version_id: ::prost::alloc::string::String,
    /// Additional authenticated data (AAD context), optional.
    /// If specified, this data will be required for decryption with the \[SymmetricDecryptRequest\].
    /// Should be encoded with base64.
    #[prost(bytes="vec", tag="3")]
    pub aad_context: ::prost::alloc::vec::Vec<u8>,
    /// Encryption algorithm and key length for the generated data key.
    #[prost(enumeration="SymmetricAlgorithm", tag="4")]
    pub data_key_spec: i32,
    /// If `true`, the method won't return the data key as plaintext.
    /// Default value is `false`.
    #[prost(bool, tag="5")]
    pub skip_plaintext: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateDataKeyResponse {
    /// ID of the symmetric KMS key that was used to encrypt the generated data key.
    #[prost(string, tag="1")]
    pub key_id: ::prost::alloc::string::String,
    /// ID of the key version that was used for encryption.
    #[prost(string, tag="2")]
    pub version_id: ::prost::alloc::string::String,
    /// Generated data key as plaintext.
    /// The field is empty, if the \[GenerateDataKeyRequest.skip_plaintext\] parameter
    /// was set to `true`.
    #[prost(bytes="vec", tag="3")]
    pub data_key_plaintext: ::prost::alloc::vec::Vec<u8>,
    /// The encrypted data key.
    #[prost(bytes="vec", tag="4")]
    pub data_key_ciphertext: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SymmetricReEncryptRequest {
    /// ID of the new key to be used for encryption.
    #[prost(string, tag="1")]
    pub key_id: ::prost::alloc::string::String,
    /// ID of the version of the new key to be used for encryption.
    /// Defaults to the primary version if not specified.
    #[prost(string, tag="2")]
    pub version_id: ::prost::alloc::string::String,
    /// Additional authenticated data to be required for decryption.
    /// Should be encoded with base64.
    #[prost(bytes="vec", tag="3")]
    pub aad_context: ::prost::alloc::vec::Vec<u8>,
    /// ID of the key that the ciphertext is currently encrypted with. May be the same as for the new key.
    #[prost(string, tag="4")]
    pub source_key_id: ::prost::alloc::string::String,
    /// Additional authenticated data provided with the initial encryption request.
    /// Should be encoded with base64.
    #[prost(bytes="vec", tag="5")]
    pub source_aad_context: ::prost::alloc::vec::Vec<u8>,
    /// Ciphertext to re-encrypt.
    /// Should be encoded with base64.
    #[prost(bytes="vec", tag="6")]
    pub ciphertext: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SymmetricReEncryptResponse {
    /// ID of the key that the ciphertext is encrypted with now.
    #[prost(string, tag="1")]
    pub key_id: ::prost::alloc::string::String,
    /// ID of key version that was used for encryption.
    #[prost(string, tag="2")]
    pub version_id: ::prost::alloc::string::String,
    /// ID of the key that the ciphertext was encrypted with previously.
    #[prost(string, tag="3")]
    pub source_key_id: ::prost::alloc::string::String,
    /// ID of the key version that was used to decrypt the re-encrypted ciphertext.
    #[prost(string, tag="4")]
    pub source_version_id: ::prost::alloc::string::String,
    /// Resulting re-encrypted ciphertext.
    #[prost(bytes="vec", tag="5")]
    pub ciphertext: ::prost::alloc::vec::Vec<u8>,
}
/// Generated client implementations.
pub mod symmetric_crypto_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Set of methods that perform symmetric encryption and decryption.
    #[derive(Debug, Clone)]
    pub struct SymmetricCryptoServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl SymmetricCryptoServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> SymmetricCryptoServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Default + Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> SymmetricCryptoServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            SymmetricCryptoServiceClient::new(
                InterceptedService::new(inner, interceptor),
            )
        }
        /// Compress requests with `gzip`.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        /// Enable decompressing responses with `gzip`.
        #[must_use]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        /// Encrypts given plaintext with the specified key.
        pub async fn encrypt(
            &mut self,
            request: impl tonic::IntoRequest<super::SymmetricEncryptRequest>,
        ) -> Result<tonic::Response<super::SymmetricEncryptResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/yandex.cloud.kms.v1.SymmetricCryptoService/Encrypt",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Decrypts the given ciphertext with the specified key.
        pub async fn decrypt(
            &mut self,
            request: impl tonic::IntoRequest<super::SymmetricDecryptRequest>,
        ) -> Result<tonic::Response<super::SymmetricDecryptResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/yandex.cloud.kms.v1.SymmetricCryptoService/Decrypt",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Re-encrypts a ciphertext with the specified KMS key.
        pub async fn re_encrypt(
            &mut self,
            request: impl tonic::IntoRequest<super::SymmetricReEncryptRequest>,
        ) -> Result<tonic::Response<super::SymmetricReEncryptResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/yandex.cloud.kms.v1.SymmetricCryptoService/ReEncrypt",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Generates a new symmetric data encryption key (not a KMS key) and returns
        /// the generated key as plaintext and as ciphertext encrypted with the specified symmetric KMS key.
        pub async fn generate_data_key(
            &mut self,
            request: impl tonic::IntoRequest<super::GenerateDataKeyRequest>,
        ) -> Result<tonic::Response<super::GenerateDataKeyResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/yandex.cloud.kms.v1.SymmetricCryptoService/GenerateDataKey",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
