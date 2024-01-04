#[derive(PartialOrd)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommondRequest {
    #[prost(oneof = "commond_request::RequestData", tags = "1, 2, 3, 4, 5, 6, 7, 8, 9")]
    pub request_data: ::core::option::Option<commond_request::RequestData>,
}
/// Nested message and enum types in `CommondRequest`.
pub mod commond_request {
    #[derive(PartialOrd)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum RequestData {
        #[prost(message, tag = "1")]
        Get(super::Get),
        #[prost(message, tag = "2")]
        Set(super::Set),
        #[prost(message, tag = "3")]
        Hget(super::HGet),
        #[prost(message, tag = "4")]
        Hset(super::HSet),
        #[prost(message, tag = "5")]
        Sadd(super::SAdd),
        #[prost(message, tag = "6")]
        Smembers(super::SMembers),
        #[prost(message, tag = "7")]
        Lpush(super::LPush),
        #[prost(message, tag = "8")]
        Lpop(super::LPop),
        #[prost(message, tag = "9")]
        Lrange(super::LRange),
    }
}
#[derive(PartialOrd)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Get {
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
}
#[derive(PartialOrd)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Set {
    #[prost(message, optional, tag = "1")]
    pub kv: ::core::option::Option<Kv>,
}
#[derive(PartialOrd)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HSet {
    #[prost(message, optional, tag = "1")]
    pub data: ::core::option::Option<HMap>,
}
#[derive(PartialOrd)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HGet {
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub field: ::prost::alloc::string::String,
}
#[derive(PartialOrd)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SAdd {
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "2")]
    pub values: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(PartialOrd)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SMembers {
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
}
#[derive(PartialOrd)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommandResponse {
    #[prost(uint32, tag = "1")]
    pub status: u32,
    #[prost(string, tag = "2")]
    pub message: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "3")]
    pub pairs: ::prost::alloc::vec::Vec<Kv>,
    #[prost(message, repeated, tag = "4")]
    pub values: ::prost::alloc::vec::Vec<Value>,
}
#[derive(PartialOrd)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Kv {
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub value: ::prost::alloc::string::String,
}
#[derive(PartialOrd)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Value {
    #[prost(string, tag = "1")]
    pub val: ::prost::alloc::string::String,
}
#[derive(PartialOrd)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HMap {
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub field_values: ::prost::alloc::vec::Vec<Kv>,
}
#[derive(PartialOrd)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LPush {
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "2")]
    pub elements: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(PartialOrd)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LPop {
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub count: i64,
}
#[derive(PartialOrd)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LRange {
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub start: i64,
    #[prost(int64, tag = "3")]
    pub stop: i64,
}
