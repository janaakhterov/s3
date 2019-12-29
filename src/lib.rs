pub use crate::client::Client;
pub use crate::error::Error;
pub use crate::get_object::GetObject;
pub use crate::gmt::Gmt;
pub use crate::headers::Headers;
pub use crate::put_object::PutObject;
pub use crate::region::Region;
pub use crate::s3_request::S3Request;
pub(crate) use crate::sign_request::sign_request;
pub use crate::signing_key::SigningKey;
pub use crate::storage_class::StorageClass;

mod client;
mod error;
mod gmt;
mod headers;
mod region;
mod s3_request;
mod sign_request;
mod signing_key;
mod storage_class;

mod get_object;
mod put_object;
