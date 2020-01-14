#[macro_use]
extern crate nom;

pub(crate) use crate::{
    aws_request::AwsRequest,
    aws_response::AwsResponse,
    gmt::Gmt,
    grant::{
        GrantType,
        GrantValue,
        OptionalGrants,
    },
    headers::Headers,
    host::Host,
    optional_header::OptionalHeader,
    payload_hash::PayloadHash,
    sign_request::SignRequest,
    signing_key::SigningKey,
};

pub use crate::{
    acl::Acl,
    cache::CacheControl,
    create_bucket::CreateBucket,
    delete_bucket::DeleteBucket,
    delete_object::DeleteObject,
    error::Error,
    get_object::GetObject,
    list_buckets::ListBuckets,
    put_object::PutObject,
    region::Region,
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
mod region;
mod sign_request;
mod signing_key;
mod storage_class;

mod create_bucket;
mod delete_bucket;
mod delete_object;
mod get_object;
mod list_buckets;
mod put_object;

mod parser;

pub mod client;
