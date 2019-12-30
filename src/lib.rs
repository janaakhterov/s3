pub(crate) use crate::{
    aws_request::AwsRequest,
    aws_response::AwsResponse,
    gmt::Gmt,
    headers::Headers,
    host::Host,
    option_header::OptionHeader,
    sign_request::SignRequest,
    signing_key::SigningKey,
};

pub use crate::{
    client::Client,
    error::Error,
    get_object::GetObject,
    put_object::PutObject,
    region::Region,
    storage_class::StorageClass,
};

mod aws_request;
mod aws_response;
mod client;
mod error;
mod gmt;
mod headers;
mod host;
mod option_header;
mod region;
mod sign_request;
mod signing_key;
mod storage_class;

mod get_object;
mod put_object;
