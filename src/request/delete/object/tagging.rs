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
use url::Url;

pub struct DeleteObjectTagging<T: AsRef<str>, V: AsRef<str>>(SubResource<T, V>);

impl<T: AsRef<str>, V: AsRef<str>> DeleteObjectTagging<T, V> {
    /// Create a new DeleteObjectTagging request with default parameters
    pub fn new(bucket: T, key: String) -> Self {
        DeleteObjectTagging(SubResource {
            bucket,
            method: Method::DELETE,
            key: Some(key),
            params: vec![(QueryParameter::TAGGING, None)],
        })
    }

    pub fn version_id(mut self, version_id: V) -> Self {
        self.0
            .params
            .push((QueryParameter::VERSION_ID, Some(version_id)));
        self
    }
}

impl<T: AsRef<str>, V: AsRef<str>> AwsRequest for DeleteObjectTagging<T, V> {
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
