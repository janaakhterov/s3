pub use crate::buf_mut::BufMut;
pub use crate::client::Client;
pub use crate::error::Error;
pub use crate::get_object::GetObject;
pub use crate::headers::Headers;
pub use crate::region::Region;
pub use crate::s3_request::S3Request;
pub(crate) use crate::sign_request::sign_request;
pub use crate::signing_key::SigningKey;
pub use crate::storage_class::StorageClass;

mod buf_mut;
mod client;
mod error;
mod headers;
mod region;
mod s3_request;
mod sign_request;
mod signing_key;
mod storage_class;

mod get_object;
