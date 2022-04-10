#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MapKeySpec {
    #[prost(string, tag="1")]
    pub value: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub pattern: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub length: ::prost::alloc::string::String,
}
