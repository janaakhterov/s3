use crate::{
    AwsRequest,
    Error,
    QueryParameter,
    Region,
    SigningKey,
    SubResource,
};
use futures_core::future::BoxFuture;
use hyper::{
    Body as HttpBody,
    Method,
    Request,
    Response,
};
use serde::Serialize;
use url::Url;

#[derive(Default, Debug, Serialize)]
struct Rule {
    #[serde(rename = "SSEAlgorithm")]
    sse: Option<String>,
    #[serde(rename = "KMSMasterKeyID")]
    kms_key: Option<String>,
}

pub struct DeleteBucketPolicy<T: AsRef<str>, V: AsRef<str>>(SubResource<T, V>);

impl<T: AsRef<str>, V: AsRef<str>> DeleteBucketPolicy<T, V> {
    /// Create a new DeleteBucketPolicy request with default parameters
    pub fn new(bucket: T) -> Self {
        DeleteBucketPolicy(SubResource {
            bucket,
            method: Method::DELETE,
            key: None,
            params: vec![(QueryParameter::POLICY, None)],
        })
    }
}

impl<T: AsRef<str>, V: AsRef<str>> AwsRequest for DeleteBucketPolicy<T, V> {
    type Response = ();

    fn into_request<AR: AsRef<str>>(
        self,
        url: Url,
        access_key: AR,
        signing_key: &SigningKey,
        region: Region,
    ) -> Result<Request<HttpBody>, Error> {
        self.0.into_request(url, access_key, signing_key, region)
    }

    fn into_response(
        response: Response<HttpBody>,
    ) -> BoxFuture<'static, Result<Self::Response, Error>> {
        SubResource::<T, V>::into_response(response)
    }
}
