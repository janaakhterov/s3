pub(crate) use crate::{
    aws_request::AwsRequest,
    aws_response::AwsResponse,
    gmt::Gmt,
    grant::{
        Permission,
        Grantee,
        OptionalGrants,
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
    cache::CacheControl,
    error::Error,
    region::Region,
    request::{
        create_bucket::CreateBucket,
        delete::{
            bucket::{
                encryption::DeleteBucketEncryption,
                inventory_config::DeleteBucketInventoryConfig,
                metrics_config::DeleteBucketMetricsConfig,
                policy::DeleteBucketPolicy,
                replication::DeleteBucketReplication,
                tagging::DeleteBucketTagging,
                website::DeleteBucketWebsite,
                DeleteBucket,
            },
            object::{
                tagging::DeleteObjectTagging,
                DeleteObject,
            },
            public_access_block::DeletePublicAccessBlock,
        },
        get::{
            bucket::{
                accelerate_config::GetBucketAccelerateConfig,
                acl::GetBucketAcl,
            },
            object::GetObject,
        },
        list_buckets::ListBuckets,
        put_bucket_encryption::PutBucketEncryption,
        put_object::PutObject,
    },
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

#[cfg(feature = "credential_file")]
mod parser;

pub mod client;
