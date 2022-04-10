/// Operation is annotation for rpc that returns longrunning operation, describes
/// message types that will be returned in metadata \[google.protobuf.Any\], and
/// in response \[google.protobuf.Any\] (for successful operation).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Operation {
    /// Optional. If present, rpc returns operation which metadata field will
    /// contains message of specified type.
    ///
    /// Optional.
    #[prost(string, tag="1")]
    pub metadata: ::prost::alloc::string::String,
    /// Required. rpc returns operation, in case of success response will contains message of
    /// specified field.
    ///
    /// Required.
    #[prost(string, tag="2")]
    pub response: ::prost::alloc::string::String,
}
