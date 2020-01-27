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

pub struct DeleteBucketMetricsConfig<T: AsRef<str>, V: AsRef<str>>(SubResource<T, V>);

impl<T: AsRef<str>, V: AsRef<str>> DeleteBucketMetricsConfig<T, V> {
    /// Create a new DeleteBucketMetricsConfig request with default parameters
    pub fn new(bucket: T, metrics_id: V) -> Self {
        DeleteBucketMetricsConfig(SubResource {
            bucket,
            method: Method::DELETE,
            key: None,
            params: vec![
                (QueryParameter::METRICS, None),
                (QueryParameter::ID, Some(metrics_id)),
            ],
        })
    }
}

impl<T: AsRef<str>, V: AsRef<str>> AwsRequest for DeleteBucketMetricsConfig<T, V> {
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
