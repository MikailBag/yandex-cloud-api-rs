#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Subject {
    /// ID of the subject.
    ///
    /// It can contain one of the following values:
    /// * `allAuthenticatedUsers`: A special system identifier that represents anyone
    ///    who is authenticated. It can be used only if the \[type\] is `system`.
    /// * `allUsers`: A special system identifier that represents anyone. No authentication is required.
    ///    For example, you don't need to specify the IAM token in an API query.
    /// * `<cloud generated id>`: An identifier that represents a user account.
    ///    It can be used only if the \[type\] is `userAccount`, `federatedUser` or `serviceAccount`.
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    /// Type of the subject.
    ///
    /// It can contain one of the following values:
    /// * `userAccount`: An account on Yandex or Yandex Connect, added to Yandex Cloud.
    /// * `serviceAccount`: A service account. This type represents the \[yandex.cloud.iam.v1.ServiceAccount\] resource.
    /// * `federatedUser`: A federated account. This type represents a user from an identity federation, like Active Directory.
    /// * `system`: System group. This type represents several accounts with a common system identifier.
    ///
    /// For more information, see [Subject to which the role is assigned](/docs/iam/concepts/access-control/#subject).
    #[prost(string, tag="2")]
    pub r#type: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccessBinding {
    /// ID of the \[yandex.cloud.iam.v1.Role\] that is assigned to the \[subject\].
    #[prost(string, tag="1")]
    pub role_id: ::prost::alloc::string::String,
    /// Identity for which access binding is being created.
    /// It can represent an account with a unique ID or several accounts with a system identifier.
    #[prost(message, optional, tag="2")]
    pub subject: ::core::option::Option<Subject>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAccessBindingsRequest {
    /// ID of the resource to list access bindings for.
    ///
    /// To get the resource ID, use a corresponding List request.
    /// For example, use the \[yandex.cloud.resourcemanager.v1.CloudService.List\] request to get the Cloud resource ID.
    #[prost(string, tag="1")]
    pub resource_id: ::prost::alloc::string::String,
    /// The maximum number of results per page that should be returned. If the number of available
    /// results is larger than \[page_size\],
    /// the service returns a \[ListAccessBindingsResponse.next_page_token\]
    /// that can be used to get the next page of results in subsequent list requests.
    /// Default value: 100.
    #[prost(int64, tag="2")]
    pub page_size: i64,
    /// Page token. Set \[page_token\]
    /// to the \[ListAccessBindingsResponse.next_page_token\]
    /// returned by a previous list request to get the next page of results.
    #[prost(string, tag="3")]
    pub page_token: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAccessBindingsResponse {
    /// List of access bindings for the specified resource.
    #[prost(message, repeated, tag="1")]
    pub access_bindings: ::prost::alloc::vec::Vec<AccessBinding>,
    /// This token allows you to get the next page of results for list requests. If the number of results
    /// is larger than \[ListAccessBindingsRequest.page_size\], use
    /// the \[next_page_token\] as the value
    /// for the \[ListAccessBindingsRequest.page_token\] query parameter
    /// in the next list request. Each subsequent list request will have its own
    /// \[next_page_token\] to continue paging through the results.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetAccessBindingsRequest {
    /// ID of the resource for which access bindings are being set.
    ///
    /// To get the resource ID, use a corresponding List request.
    #[prost(string, tag="1")]
    pub resource_id: ::prost::alloc::string::String,
    /// Access bindings to be set. For more information, see [Access Bindings](/docs/iam/concepts/access-control/#access-bindings).
    #[prost(message, repeated, tag="2")]
    pub access_bindings: ::prost::alloc::vec::Vec<AccessBinding>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetAccessBindingsMetadata {
    /// ID of the resource for which access bindings are being set.
    #[prost(string, tag="1")]
    pub resource_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateAccessBindingsRequest {
    /// ID of the resource for which access bindings are being updated.
    #[prost(string, tag="1")]
    pub resource_id: ::prost::alloc::string::String,
    /// Updates to access bindings.
    #[prost(message, repeated, tag="2")]
    pub access_binding_deltas: ::prost::alloc::vec::Vec<AccessBindingDelta>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateAccessBindingsMetadata {
    /// ID of the resource for which access bindings are being updated.
    #[prost(string, tag="1")]
    pub resource_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccessBindingDelta {
    /// The action that is being performed on an access binding.
    #[prost(enumeration="AccessBindingAction", tag="1")]
    pub action: i32,
    /// Access binding. For more information, see [Access Bindings](/docs/iam/concepts/access-control/#access-bindings).
    #[prost(message, optional, tag="2")]
    pub access_binding: ::core::option::Option<AccessBinding>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AccessBindingAction {
    Unspecified = 0,
    /// Addition of an access binding.
    Add = 1,
    /// Removal of an access binding.
    Remove = 2,
}
