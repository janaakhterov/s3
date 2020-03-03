use crate::{
    AwsRequest,
    Error,
    QueryParameter,
    Region,
    SigningKey,
    request::sub_resource::SubResource,
};
use futures_core::future::BoxFuture;
use http::method::Method;
use hyper::{
    Body as HttpBody,
    Request,
    Response,
};
use url::Url;

pub struct GetBucketPolicy<'a>(SubResource<'a>);

impl<'a> GetBucketPolicy<'a> {
    /// Create a new GetBucketPolicy request with default parameters
    pub fn new(bucket: &'a str) -> Self {
        GetBucketPolicy(SubResource {
            bucket,
            method: Method::GET,
            key: None,
            params: vec![(QueryParameter::POLICY, None)],
        })
    }
}

impl<'a> AwsRequest for GetBucketPolicy<'a> {
    type Response = String;

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
            let string = String::from_utf8_lossy(&bytes).into_owned();

            Ok(string)
        })
    }
}
