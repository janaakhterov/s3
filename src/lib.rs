pub(crate) use crate::{
    aws_response::AwsResponse,
    gmt::Gmt,
    grant::{
        Grantee,
        OptionalGrants,
        Permission,
    },
    headers::Headers,
    host::Host,
    optional_header::OptionalHeader,
    payload_hash::PayloadHash,
    query::{
        QueryParam,
        QueryParameter,
    },
    request::sub_resource::SubResource,
    sign_request::SignRequest,
    signing_key::SigningKey,
};

pub use crate::{
    acl::Acl,
    aws_request::AwsRequest,
    cache::CacheControl,
    client::Client,
    error::{
        Error,
        Result,
    },
    region::Region,
    request::*,
    storage_class::StorageClass,
};

mod acl;
mod aws_request;
mod aws_response;
mod cache;
mod error;
mod gmt;
mod grant;
mod headers;
mod host;
mod optional_header;
mod payload_hash;
mod query;
mod region;
mod request;
mod sign_request;
mod signing_key;
mod storage_class;
mod types;

#[cfg(feature = "credential_file")]
mod parser;

pub mod client;
