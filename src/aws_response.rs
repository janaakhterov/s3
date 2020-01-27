use crate::{
    error::ResponseError,
    storage_class::StorageClass,
    Error,
    Gmt,
    Headers,
};
use chrono::{
    DateTime,
    Utc,
};
use futures_core::future::BoxFuture;
use http::HeaderValue;
use http_body::Body;
use hyper::{
    Body as HttpBody,
    Response,
};
use std::str::FromStr;

pub trait AwsResponse {
    fn last_modified(&self) -> Result<DateTime<Utc>, Error>;
    fn etag(&self) -> Result<String, Error>;
    fn version_id(&self) -> Result<Option<String>, Error>;
    fn expires(&self) -> Result<Option<DateTime<Utc>>, Error>;
    fn storage_class(&self) -> Result<StorageClass, Error>;
    fn parts_count(&self) -> Result<Option<u64>, Error>;
    fn error(&mut self) -> BoxFuture<Result<(), Error>>;
    fn delete_marker(&mut self) -> Result<Option<bool>, Error>;
}

impl AwsResponse for Response<HttpBody> {
    fn last_modified(&self) -> Result<DateTime<Utc>, Error> {
        Ok(self
            .headers()
            .get(Headers::LAST_MODIFIED)
            .map(HeaderValue::to_str)
            .transpose()?
            .map(DateTime::from_gmt)
            .transpose()?
            .ok_or(Error::LastModifiedNotPresentOnGetResponse)?)
    }

    fn etag(&self) -> Result<String, Error> {
        Ok(self
            .headers()
            .get(Headers::ETAG)
            .ok_or(Error::NoEtagInResponse)?
            .to_str()?
            .to_owned())
    }

    fn version_id(&self) -> Result<Option<String>, Error> {
        Ok(self
            .headers()
            .get(Headers::X_AMZ_VERSION_ID)
            .map(HeaderValue::to_str)
            .transpose()?
            .map(str::to_owned))
    }

    fn expires(&self) -> Result<Option<DateTime<Utc>>, Error> {
        Ok(self
            .headers()
            .get(Headers::EXPIRES)
            .map(HeaderValue::to_str)
            .transpose()?
            .map(DateTime::from_gmt)
            .transpose()?)
    }

    fn storage_class(&self) -> Result<StorageClass, Error> {
        if let Some(header) = self.headers().get(Headers::X_AMZ_STORAGE_CLASS) {
            Ok(StorageClass::from_str(header.to_str()?)?)
        } else {
            Ok(StorageClass::Standard)
        }
    }

    fn parts_count(&self) -> Result<Option<u64>, Error> {
        Ok(self
            .headers()
            .get(Headers::PARTS_COUNT)
            .map(HeaderValue::to_str)
            .transpose()?
            .map(u64::from_str)
            .transpose()?)
    }

    fn error(&mut self) -> BoxFuture<Result<(), Error>> {
        Box::pin(async move {
            if !self.status().is_success() {
                let mut bytes: Vec<u8> = Vec::new();

                while let Some(next) = self.data().await {
                    let chunk = next?;
                    bytes.extend_from_slice(&chunk);
                }

                if bytes.is_empty() {
                    Err(Error::StatusCode(self.status()))
                } else {
                    let error = String::from_utf8_lossy(&bytes);
                    let error: ResponseError = quick_xml::de::from_str(&error)?;
                    Err(Error::ResponseError(error))
                }
            } else {
                Ok(())
            }
        })
    }

    fn delete_marker(&mut self) -> Result<Option<bool>, Error> {
        Ok(self
            .headers()
            .get(Headers::DELETE_MARKER)
            .map(HeaderValue::to_str)
            .transpose()?
            .map(bool::from_str)
            .transpose()?)
    }
}
