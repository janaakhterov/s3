use crate::{
    Request,
    AwsRequest,
    AwsResponse,
    Error,
    Region,
    SigningKey,
};
use futures_core::future::BoxFuture;
use http::method::Method;
use hyper::{
    Body as HttpBody,
    Request as HttpRequest,
    Response,
};
use url::Url;

pub mod tagging;

pub use tagging::*;

pub struct DeleteObject<'a> {
    /// The bucket name of the bucket containing the object.
    pub bucket: &'a str,

    /// Key name of the object to delete.
    pub key: &'a str,
}

impl<'a> DeleteObject<'a> {
    /// Creates a new DeleteObject request with the given bucket and key
    pub fn new(bucket: &'a str, key: &'a str) -> Self {
        DeleteObject { bucket, key }
    }
}

impl<'a> AwsRequest for DeleteObject<'a> {
    type Response = bool;

    fn into_request<AR: AsRef<str>>(
        self,
        url: Url,
        access_key: AR,
        signing_key: &SigningKey,
        region: Region,
    ) -> Result<HttpRequest<HttpBody>, Error> {
        Request::new(Method::DELETE)
            .bucket(self.bucket)
            .key(self.key)
            .host(url.clone())?
            .region(region)
            .build(&access_key.as_ref(), &signing_key)
    }

    fn into_response(
        mut response: Response<HttpBody>,
    ) -> BoxFuture<'static, Result<Self::Response, Error>> {
        Box::pin(async move {
            response.error().await?;

            let deleted = response.delete_marker()?.unwrap_or(false);

            Ok(deleted)
        })
    }
}
