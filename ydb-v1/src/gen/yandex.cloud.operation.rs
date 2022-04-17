/// An Operation resource. For more information, see \[Operation\](/docs/api-design-guide/concepts/operation).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Operation {
    /// ID of the operation.
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    /// Description of the operation. 0-256 characters long.
    ///
    /// ex: Create VM, Stop VM, Delete Disk, Snapshot Disk, etc
    #[prost(string, tag="2")]
    pub description: ::prost::alloc::string::String,
    /// Creation timestamp.
    #[prost(message, optional, tag="3")]
    pub created_at: ::core::option::Option<::prost_types::Timestamp>,
    /// ID of the user or service account who initiated the operation.
    #[prost(string, tag="4")]
    pub created_by: ::prost::alloc::string::String,
    /// The time when the Operation resource was last modified.
    #[prost(message, optional, tag="5")]
    pub modified_at: ::core::option::Option<::prost_types::Timestamp>,
    /// If the value is `false`, it means the operation is still in progress.
    /// If `true`, the operation is completed, and either `error` or `response` is available.
    #[prost(bool, tag="6")]
    pub done: bool,
    /// Service-specific metadata associated with the operation.
    /// It typically contains the ID of the target resource that the operation is performed on.
    /// Any method that returns a long-running operation should document the metadata type, if any.
    #[prost(message, optional, tag="7")]
    pub metadata: ::core::option::Option<::prost_types::Any>,
    /// The operation result.
    /// If `done == false` and there was no failure detected, neither `error` nor `response` is set.
    /// If `done == false` and there was a failure detected, `error` is set.
    /// If `done == true`, exactly one of `error` or `response` is set.
    #[prost(oneof="operation::Result", tags="8, 9")]
    pub result: ::core::option::Option<operation::Result>,
}
/// Nested message and enum types in `Operation`.
pub mod operation {
    /// The operation result.
    /// If `done == false` and there was no failure detected, neither `error` nor `response` is set.
    /// If `done == false` and there was a failure detected, `error` is set.
    /// If `done == true`, exactly one of `error` or `response` is set.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Result {
        /// The error result of the operation in case of failure or cancellation.
        #[prost(message, tag="8")]
        Error(super::super::super::super::google::rpc::Status),
        /// The normal response of the operation in case of success.
        /// If the original method returns no data on success, such as Delete,
        /// the response is \[google.protobuf.Empty\].
        /// If the original method is the standard Create/Update,
        /// the response should be the target resource of the operation.
        /// Any method that returns a long-running operation should document the response type, if any.
        #[prost(message, tag="9")]
        Response(::prost_types::Any),
    }
}
