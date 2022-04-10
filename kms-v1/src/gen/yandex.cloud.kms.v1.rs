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
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateSymmetricKeyRequest {
    /// ID of the folder to create a symmetric KMS key in.
    #[prost(string, tag="1")]
    pub folder_id: ::prost::alloc::string::String,
    /// Name of the key.
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    /// Description of the key.
    #[prost(string, tag="3")]
    pub description: ::prost::alloc::string::String,
    /// Custom labels for the symmetric KMS key as `key:value` pairs. Maximum 64 per key.
    /// For example, `"project": "mvp"` or `"source": "dictionary"`.
    #[prost(map="string, string", tag="4")]
    pub labels: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Encryption algorithm to be used with a new key version, generated with the next rotation.
    #[prost(enumeration="SymmetricAlgorithm", tag="5")]
    pub default_algorithm: i32,
    /// Interval between automatic rotations. To disable automatic rotation, don't include
    /// this field in the creation request.
    #[prost(message, optional, tag="6")]
    pub rotation_period: ::core::option::Option<::prost_types::Duration>,
    /// Flag that inhibits deletion of the symmetric KMS key
    #[prost(bool, tag="7")]
    pub deletion_protection: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateSymmetricKeyMetadata {
    /// ID of the key being created.
    #[prost(string, tag="1")]
    pub key_id: ::prost::alloc::string::String,
    /// ID of the primary version of the key being created.
    #[prost(string, tag="2")]
    pub primary_version_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSymmetricKeyRequest {
    /// ID of the symmetric KMS key to return.
    /// To get the ID of a symmetric KMS key use a \[SymmetricKeyService.List\] request.
    #[prost(string, tag="1")]
    pub key_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSymmetricKeysRequest {
    /// ID of the folder to list symmetric KMS keys in.
    #[prost(string, tag="1")]
    pub folder_id: ::prost::alloc::string::String,
    /// The maximum number of results per page to return. If the number of available
    /// results is larger than \[page_size\], the service returns a \[ListSymmetricKeysResponse.next_page_token\]
    /// that can be used to get the next page of results in subsequent list requests.
    /// Default value: 100.
    #[prost(int64, tag="2")]
    pub page_size: i64,
    /// Page token. To get the next page of results, set \[page_token\] to the
    /// \[ListSymmetricKeysResponse.next_page_token\] returned by a previous list request.
    #[prost(string, tag="3")]
    pub page_token: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSymmetricKeysResponse {
    /// List of symmetric KMS keys in the specified folder.
    #[prost(message, repeated, tag="1")]
    pub keys: ::prost::alloc::vec::Vec<SymmetricKey>,
    /// This token allows you to get the next page of results for list requests. If the number
    /// of results is greater than the specified \[ListSymmetricKeysRequest.page_size\], use
    /// the \[next_page_token\] as the value for the \[ListSymmetricKeysRequest.page_token\] query parameter
    /// in the next list request. Each subsequent list request will have its own
    /// \[next_page_token\] to continue paging through the results.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSymmetricKeyVersionsRequest {
    /// ID of the symmetric KMS key to list versions for.
    #[prost(string, tag="1")]
    pub key_id: ::prost::alloc::string::String,
    /// The maximum number of results per page to return. If the number of available
    /// results is larger than \[page_size\], the service returns a \[ListSymmetricKeyVersionsResponse.next_page_token\]
    /// that can be used to get the next page of results in subsequent list requests.
    /// Default value: 100.
    #[prost(int64, tag="2")]
    pub page_size: i64,
    /// Page token. To get the next page of results, set \[page_token\] to the
    /// \[ListSymmetricKeyVersionsResponse.next_page_token\] returned by a previous list request.
    #[prost(string, tag="3")]
    pub page_token: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSymmetricKeyVersionsResponse {
    /// List of versions for the specified symmetric KMS key.
    #[prost(message, repeated, tag="1")]
    pub key_versions: ::prost::alloc::vec::Vec<SymmetricKeyVersion>,
    /// This token allows you to get the next page of results for list requests. If the number
    /// of results is greater than the specified \[ListSymmetricKeyVersionsRequest.page_size\], use
    /// the \[next_page_token\] as the value for the \[ListSymmetricKeyVersionsRequest.page_token\] query parameter
    /// in the next list request. Each subsequent list request will have its own
    /// \[next_page_token\] to continue paging through the results.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateSymmetricKeyRequest {
    /// ID of the symmetric KMS key to update.
    /// To get the ID of a symmetric KMS key use a \[SymmetricKeyService.List\] request.
    #[prost(string, tag="1")]
    pub key_id: ::prost::alloc::string::String,
    /// Field mask that specifies which attributes of the symmetric KMS key are going to be updated.
    #[prost(message, optional, tag="2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// New name for the symmetric KMS key.
    #[prost(string, tag="3")]
    pub name: ::prost::alloc::string::String,
    /// New description for the symmetric KMS key.
    #[prost(string, tag="4")]
    pub description: ::prost::alloc::string::String,
    /// New status for the symmetric KMS key.
    /// Using the \[SymmetricKeyService.Update\] method you can only set ACTIVE or INACTIVE status.
    #[prost(enumeration="symmetric_key::Status", tag="5")]
    pub status: i32,
    /// Custom labels for the symmetric KMS key as `key:value` pairs. Maximum 64 per key.
    #[prost(map="string, string", tag="6")]
    pub labels: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Default encryption algorithm to be used with new versions of the symmetric KMS key.
    #[prost(enumeration="SymmetricAlgorithm", tag="7")]
    pub default_algorithm: i32,
    /// Time period between automatic symmetric KMS key rotations.
    ///
    /// period between two automatic rotations
    #[prost(message, optional, tag="8")]
    pub rotation_period: ::core::option::Option<::prost_types::Duration>,
    /// Flag that inhibits deletion of the symmetric KMS key
    #[prost(bool, tag="9")]
    pub deletion_protection: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateSymmetricKeyMetadata {
    /// ID of the key being updated.
    #[prost(string, tag="1")]
    pub key_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteSymmetricKeyRequest {
    /// ID of the key to be deleted.
    #[prost(string, tag="1")]
    pub key_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteSymmetricKeyMetadata {
    /// ID of the key being deleted.
    #[prost(string, tag="1")]
    pub key_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetPrimarySymmetricKeyVersionRequest {
    /// ID of the key to set a primary version for.
    #[prost(string, tag="1")]
    pub key_id: ::prost::alloc::string::String,
    /// ID of the version that should become primary for the specified key.
    #[prost(string, tag="2")]
    pub version_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetPrimarySymmetricKeyVersionMetadata {
    /// ID of the key that the primary version if being changed for.
    #[prost(string, tag="1")]
    pub key_id: ::prost::alloc::string::String,
    /// ID of the version that is being made primary for the key.
    #[prost(string, tag="2")]
    pub version_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RotateSymmetricKeyRequest {
    /// ID of the key to be rotated.
    #[prost(string, tag="1")]
    pub key_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RotateSymmetricKeyMetadata {
    /// ID of the key being rotated.
    #[prost(string, tag="1")]
    pub key_id: ::prost::alloc::string::String,
    /// ID of the version generated as a result of key rotation.
    #[prost(string, tag="2")]
    pub new_primary_version_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScheduleSymmetricKeyVersionDestructionRequest {
    /// ID of the key whose version should be scheduled for destruction.
    #[prost(string, tag="1")]
    pub key_id: ::prost::alloc::string::String,
    /// ID of the version to be destroyed.
    #[prost(string, tag="2")]
    pub version_id: ::prost::alloc::string::String,
    /// Time interval between the version destruction request and actual destruction.
    /// Default value: 7 days.
    #[prost(message, optional, tag="3")]
    pub pending_period: ::core::option::Option<::prost_types::Duration>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScheduleSymmetricKeyVersionDestructionMetadata {
    /// ID of the key whose version is being scheduled for destruction.
    #[prost(string, tag="1")]
    pub key_id: ::prost::alloc::string::String,
    /// ID of the version that is being scheduled for destruction.
    #[prost(string, tag="2")]
    pub version_id: ::prost::alloc::string::String,
    /// Time when the version is scheduled to be destroyed.
    #[prost(message, optional, tag="3")]
    pub destroy_at: ::core::option::Option<::prost_types::Timestamp>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CancelSymmetricKeyVersionDestructionRequest {
    /// ID of the key to cancel a version's destruction for.
    #[prost(string, tag="1")]
    pub key_id: ::prost::alloc::string::String,
    /// ID of the version whose scheduled destruction should be cancelled.
    #[prost(string, tag="2")]
    pub version_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CancelSymmetricKeyVersionDestructionMetadata {
    /// ID of the key whose version's destruction is being cancelled.
    #[prost(string, tag="1")]
    pub key_id: ::prost::alloc::string::String,
    /// ID of the version whose scheduled destruction is being cancelled.
    #[prost(string, tag="2")]
    pub version_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSymmetricKeyOperationsRequest {
    /// ID of the symmetric KMS key to get operations for.
    ///
    /// To get the key ID, use a \[SymmetricKeyService.List\] request.
    #[prost(string, tag="1")]
    pub key_id: ::prost::alloc::string::String,
    /// The maximum number of results per page that should be returned. If the number of available
    /// results is larger than \[page_size\], the service returns a \[ListSymmetricKeyOperationsResponse.next_page_token\]
    /// that can be used to get the next page of results in subsequent list requests.
    /// Default value: 100.
    #[prost(int64, tag="2")]
    pub page_size: i64,
    /// Page token. To get the next page of results, set \[page_token\] to the
    /// \[ListSymmetricKeyOperationsResponse.next_page_token\] returned by a previous list request.
    #[prost(string, tag="3")]
    pub page_token: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSymmetricKeyOperationsResponse {
    /// List of operations for the specified key.
    #[prost(message, repeated, tag="1")]
    pub operations: ::prost::alloc::vec::Vec<super::super::operation::Operation>,
    /// This token allows you to get the next page of results for list requests. If the number of results
    /// is larger than \[ListSymmetricKeyOperationsRequest.page_size\], use the \[next_page_token\] as the value
    /// for the \[ListSymmetricKeyOperationsRequest.page_token\] query parameter in the next list request.
    /// Each subsequent list request will have its own \[next_page_token\] to continue paging through the results.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod symmetric_key_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Set of methods for managing symmetric KMS keys.
    #[derive(Debug, Clone)]
    pub struct SymmetricKeyServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl SymmetricKeyServiceClient<tonic::transport::Channel> {
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
    impl<T> SymmetricKeyServiceClient<T>
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
        ) -> SymmetricKeyServiceClient<InterceptedService<T, F>>
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
            SymmetricKeyServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Creates a symmetric KMS key in the specified folder.
        pub async fn create(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateSymmetricKeyRequest>,
        ) -> Result<
                tonic::Response<super::super::super::operation::Operation>,
                tonic::Status,
            > {
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
                "/yandex.cloud.kms.v1.SymmetricKeyService/Create",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Returns the specified symmetric KMS key.
        ///
        ///  To get the list of available symmetric KMS keys, make a [SymmetricKeyService.List] request.
        pub async fn get(
            &mut self,
            request: impl tonic::IntoRequest<super::GetSymmetricKeyRequest>,
        ) -> Result<tonic::Response<super::SymmetricKey>, tonic::Status> {
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
                "/yandex.cloud.kms.v1.SymmetricKeyService/Get",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Returns the list of symmetric KMS keys in the specified folder.
        pub async fn list(
            &mut self,
            request: impl tonic::IntoRequest<super::ListSymmetricKeysRequest>,
        ) -> Result<tonic::Response<super::ListSymmetricKeysResponse>, tonic::Status> {
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
                "/yandex.cloud.kms.v1.SymmetricKeyService/List",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Returns the list of versions of the specified symmetric KMS key.
        pub async fn list_versions(
            &mut self,
            request: impl tonic::IntoRequest<super::ListSymmetricKeyVersionsRequest>,
        ) -> Result<
                tonic::Response<super::ListSymmetricKeyVersionsResponse>,
                tonic::Status,
            > {
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
                "/yandex.cloud.kms.v1.SymmetricKeyService/ListVersions",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates the specified symmetric KMS key.
        pub async fn update(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateSymmetricKeyRequest>,
        ) -> Result<
                tonic::Response<super::super::super::operation::Operation>,
                tonic::Status,
            > {
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
                "/yandex.cloud.kms.v1.SymmetricKeyService/Update",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes the specified symmetric KMS key. This action also automatically schedules
        /// the destruction of all of the key's versions in 72 hours.
        ///
        /// The key and its versions appear absent in [SymmetricKeyService.Get] and [SymmetricKeyService.List]
        /// requests, but can be restored within 72 hours with a request to tech support.
        pub async fn delete(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteSymmetricKeyRequest>,
        ) -> Result<
                tonic::Response<super::super::super::operation::Operation>,
                tonic::Status,
            > {
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
                "/yandex.cloud.kms.v1.SymmetricKeyService/Delete",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Sets the primary version for the specified key. The primary version is used
        /// by default for all encrypt/decrypt operations where no version ID is specified.
        pub async fn set_primary_version(
            &mut self,
            request: impl tonic::IntoRequest<super::SetPrimarySymmetricKeyVersionRequest>,
        ) -> Result<
                tonic::Response<super::super::super::operation::Operation>,
                tonic::Status,
            > {
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
                "/yandex.cloud.kms.v1.SymmetricKeyService/SetPrimaryVersion",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Schedules the specified key version for destruction.
        ///
        /// Scheduled destruction can be cancelled with the [SymmetricKeyService.CancelVersionDestruction] method.
        pub async fn schedule_version_destruction(
            &mut self,
            request: impl tonic::IntoRequest<
                super::ScheduleSymmetricKeyVersionDestructionRequest,
            >,
        ) -> Result<
                tonic::Response<super::super::super::operation::Operation>,
                tonic::Status,
            > {
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
                "/yandex.cloud.kms.v1.SymmetricKeyService/ScheduleVersionDestruction",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Cancels previously scheduled version destruction, if the version hasn't been destroyed yet.
        pub async fn cancel_version_destruction(
            &mut self,
            request: impl tonic::IntoRequest<
                super::CancelSymmetricKeyVersionDestructionRequest,
            >,
        ) -> Result<
                tonic::Response<super::super::super::operation::Operation>,
                tonic::Status,
            > {
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
                "/yandex.cloud.kms.v1.SymmetricKeyService/CancelVersionDestruction",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Rotates the specified key: creates a new key version and makes it the primary version.
        /// The old version remains available for decryption of ciphertext encrypted with it.
        pub async fn rotate(
            &mut self,
            request: impl tonic::IntoRequest<super::RotateSymmetricKeyRequest>,
        ) -> Result<
                tonic::Response<super::super::super::operation::Operation>,
                tonic::Status,
            > {
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
                "/yandex.cloud.kms.v1.SymmetricKeyService/Rotate",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists operations for the specified symmetric KMS key.
        pub async fn list_operations(
            &mut self,
            request: impl tonic::IntoRequest<super::ListSymmetricKeyOperationsRequest>,
        ) -> Result<
                tonic::Response<super::ListSymmetricKeyOperationsResponse>,
                tonic::Status,
            > {
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
                "/yandex.cloud.kms.v1.SymmetricKeyService/ListOperations",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists existing access bindings for the specified key.
        pub async fn list_access_bindings(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::access::ListAccessBindingsRequest,
            >,
        ) -> Result<
                tonic::Response<super::super::super::access::ListAccessBindingsResponse>,
                tonic::Status,
            > {
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
                "/yandex.cloud.kms.v1.SymmetricKeyService/ListAccessBindings",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Sets access bindings for the key.
        pub async fn set_access_bindings(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::access::SetAccessBindingsRequest,
            >,
        ) -> Result<
                tonic::Response<super::super::super::operation::Operation>,
                tonic::Status,
            > {
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
                "/yandex.cloud.kms.v1.SymmetricKeyService/SetAccessBindings",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates access bindings for the specified key.
        pub async fn update_access_bindings(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::access::UpdateAccessBindingsRequest,
            >,
        ) -> Result<
                tonic::Response<super::super::super::operation::Operation>,
                tonic::Status,
            > {
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
                "/yandex.cloud.kms.v1.SymmetricKeyService/UpdateAccessBindings",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
