use crate::{
    AwsRequest,
    AwsResponse,
    Error,
    Headers,
    Host,
    PayloadHash,
    QueryParam,
    QueryParameter,
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
use serde::Serialize;
use url::Url;

// DeleteBucketWebsite requset Headers, this list *MUST* be in
// sorted order as it is used in the signing process
// of each request.
const HEADERS: [&str; 3] = [
    Headers::HOST,
    Headers::X_AMZ_CONTENT_SHA256,
    Headers::X_AMZ_DATE,
];

#[derive(Default, Debug, Serialize)]
struct Rule {
    #[serde(rename = "SSEAlgorithm")]
    sse: Option<String>,
    #[serde(rename = "KMSMasterKeyID")]
    kms_key: Option<String>,
}

pub struct DeleteBucketWebsite<T: AsRef<str>> {
    /// Bucket name from which to Delete the encryption.
    pub bucket: T,
}

impl<T: AsRef<str>> DeleteBucketWebsite<T> {
    /// Create a new DeleteBucketWebsite request with default parameters
    pub fn new(bucket: T) -> Self {
        DeleteBucketWebsite { bucket }
    }
}

impl<T: AsRef<str>> AwsRequest for DeleteBucketWebsite<T> {
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
            .query_param(QueryParameter::WEBSITE, "")?
            .payload_hash(None)?
            .sign(&access_key.as_ref(), &signing_key, region.clone(), &HEADERS)?;

        Ok(request.body(HttpBody::empty())?)
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
