use crate::{
    AwsRequest,
    AwsResponse,
    Error,
    Headers,
    Host,
    PayloadHash,
    Region,
    SignRequest,
    SigningKey,
};
use futures_core::future::BoxFuture;
use http::method::Method;
use hyper::{
    Body as HttpBody,
    Request,
    Response,
};
use url::Url;

pub mod tagging;

// DeleteObject requset Headers, this list *MUST* be in
// sorted order as it is used in the signing process
// of each request.
const HEADERS: [&str; 3] = [
    Headers::HOST,
    Headers::X_AMZ_CONTENT_SHA256,
    Headers::X_AMZ_DATE,
];

pub struct DeleteObject<T: AsRef<str>> {
    /// The bucket name of the bucket containing the object.
    pub bucket: T,

    /// Key name of the object to delete.
    pub key: T,
}

impl<T: AsRef<str>> DeleteObject<T> {
    /// Creates a new DeleteObject request with the given bucket and key
    pub fn new(bucket: T, key: T) -> Self {
        DeleteObject { bucket, key }
    }
}

impl<T: AsRef<str>> AwsRequest for DeleteObject<T> {
    type Response = bool;

    fn into_request<AR: AsRef<str>>(
        self,
        url: Url,
        access_key: AR,
        signing_key: &SigningKey,
        region: Region,
    ) -> Result<Request<HttpBody>, Error> {
        let request = Request::builder()
            .method(Method::DELETE)
            .host(url, self.bucket, self.key, None)?
            .payload_hash(None)?
            .sign(&access_key.as_ref(), &signing_key, region.clone(), &HEADERS)?;

        Ok(request.body(HttpBody::empty())?)
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
