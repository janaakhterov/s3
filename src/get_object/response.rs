use crate::{
    AwsResponse,
    Error,
    StorageClass,
};
use chrono::{
    DateTime,
    Utc,
};
use futures_core::future::BoxFuture;
use http_body::Body;
use hyper::{
    body::Body as HttpBody,
    Response,
};
use std::borrow::Cow;

#[derive(Debug)]
pub struct GetObjectResponse {
    pub last_modified: DateTime<Utc>,
    pub etag: String,
    pub version_id: Option<String>,
    pub expires: Option<DateTime<Utc>>,
    pub storage_class: StorageClass,
    pub parts_count: Option<u64>,
    pub body: Vec<u8>,
}

impl GetObjectResponse {
    pub fn as_str(&self) -> Cow<str> {
        String::from_utf8_lossy(&self.body)
    }

    pub(super) fn from_response(
        mut response: Response<HttpBody>,
    ) -> BoxFuture<'static, Result<Self, Error>> {
        Box::pin(async move {
            response.error().await?;

            let last_modified = response.last_modified()?;
            let etag = response.etag()?;
            let version_id = response.version_id()?;
            let expires = response.expires()?;
            let storage_class = response.storage_class()?;
            let parts_count = response.parts_count()?;

            let mut bytes: Vec<u8> = Vec::new();

            while let Some(next) = response.data().await {
                let chunk = next?;
                bytes.extend_from_slice(&chunk);
            }

            Ok(Self {
                last_modified,
                etag,
                version_id,
                storage_class,
                expires,
                parts_count,
                body: bytes,
            })
        })
    }
}
