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

pub struct DeleteBucketWebsite<'a>(SubResource<'a>);

impl<'a> DeleteBucketWebsite<'a> {
    /// Create a new DeleteBucketWebsite request with default parameters
    pub fn new(bucket: &'a str) -> Self {
        DeleteBucketWebsite(SubResource {
            bucket,
            method: Method::DELETE,
            key: None,
            params: vec![(QueryParameter::WEBSITE, None)],
        })
    }
}

impl<'a> AwsRequest for DeleteBucketWebsite<'a> {
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
