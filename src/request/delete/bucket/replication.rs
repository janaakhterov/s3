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

pub struct DeleteBucketReplication<T: AsRef<str>, V: AsRef<str>>(SubResource<T, V>);

impl<T: AsRef<str>, V: AsRef<str>> DeleteBucketReplication<T, V> {
    /// Create a new DeleteBucketReplication request with default parameters
    pub fn new(bucket: T) -> Self {
        DeleteBucketReplication(SubResource {
            bucket,
            method: Method::DELETE,
            key: None,
            params: vec![(QueryParameter::REPLICATION, None)],
        })
    }
}

impl<T: AsRef<str>, V: AsRef<str>> AwsRequest for DeleteBucketReplication<T, V> {
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
