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

pub struct DeleteBucketMetricsConfig<'a>(SubResource<'a>);

impl<'a> DeleteBucketMetricsConfig<'a> {
    /// Create a new DeleteBucketMetricsConfig request with default parameters
    pub fn new(bucket: &'a str, metrics_id: &'a str) -> Self {
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

impl<'a> AwsRequest for DeleteBucketMetricsConfig<'a> {
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
        Box::pin(async move {
            SubResource::<'a>::into_response(response).await?;

            Ok(())
        })
    }
}
