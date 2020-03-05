use crate::{
    error,
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

pub mod encryption;
pub mod inventory_config;
pub mod metrics_config;
pub mod policy;
pub mod replication;
pub mod tagging;
pub mod website;

pub use encryption::*;
pub use inventory_config::*;
pub use metrics_config::*;
pub use policy::*;
pub use replication::*;
pub use tagging::*;
pub use website::*;

// DeleteBucket request Headers, this list *MUST* be in
// sorted order as it is used in the signing process
// of each request.
const HEADERS: [&str; 9] = [
    Headers::HOST,
    Headers::X_AMZ_ACL,
    Headers::X_AMZ_CONTENT_SHA256,
    Headers::X_AMZ_DATE,
    Headers::X_AMZ_GRANT_WRITE,
    Headers::X_AMZ_GRANT_READ,
    Headers::X_AMZ_GRANT_WRITE_ACP,
    Headers::X_AMZ_GRANT_READ_ACP,
    Headers::X_AMZ_GRANT_FULL_CONTROL,
];

pub struct DeleteBucket<'a> {
    /// The bucket to delete.
    bucket: &'a str,
}

impl<'a> DeleteBucket<'a> {
    /// Create a new DeleteBucket request with the given bucket name.
    pub fn new(bucket: &'a str) -> Self {
        DeleteBucket { bucket }
    }
}

impl<'a> AwsRequest for DeleteBucket<'a> {
    type Response = ();

    fn into_request<AR: AsRef<str>>(
        self,
        url: Url,
        access_key: AR,
        signing_key: &SigningKey,
        region: Region,
    ) -> Result<Request<HttpBody>, Error> {
        let request = Request::builder()
            .method(Method::DELETE)
            .host(url, self.bucket, "", None)?
            .payload_hash(None)?
            .sign(&access_key.as_ref(), &signing_key, region.clone(), &HEADERS)?;

        Ok(request
            .body(HttpBody::empty())
            .map_err(error::Internal::from)?)
    }

    fn into_response(
        mut response: Response<HttpBody>,
    ) -> BoxFuture<'static, Result<Self::Response, Error>> {
        Box::pin(async move {
            response.error().await?;

            Ok(())
        })
    }
}
