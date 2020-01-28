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

pub struct DeleteBucketPolicy<'a>(SubResource<'a>);

impl<'a> DeleteBucketPolicy<'a> {
    /// Create a new DeleteBucketPolicy request with default parameters
    pub fn new(bucket: &'a str) -> Self {
        DeleteBucketPolicy(SubResource {
            bucket,
            method: Method::DELETE,
            key: None,
            params: vec![(QueryParameter::POLICY, None)],
        })
    }
}

impl<'a> AwsRequest for DeleteBucketPolicy<'a> {
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
