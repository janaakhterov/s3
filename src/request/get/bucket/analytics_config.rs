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
use crate::types::BucketAnalytics;

pub struct GetBucketAnalyticsConfig<'a>(SubResource<'a>);

impl<'a> GetBucketAnalyticsConfig<'a> {
    /// Create a new GetBucketAnalyticsConfig request with default parameters
    pub fn new(bucket: &'a str, id: &'a str) -> Self {
        GetBucketAnalyticsConfig(SubResource {
            bucket,
            method: Method::GET,
            key: None,
            params: vec![
                (QueryParameter::ANALYTICS, None),
                (QueryParameter::ID, Some(id))],
        })
    }
}

impl<'a> AwsRequest for GetBucketAnalyticsConfig<'a> {
    type Response = BucketAnalytics;

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
            let bytes = SubResource::<'a>::into_response(response).await?;
            let string = String::from_utf8_lossy(&bytes);

            let resp: BucketAnalytics = quick_xml::de::from_str(&string)?;

            Ok(resp)
        })
    }
}
