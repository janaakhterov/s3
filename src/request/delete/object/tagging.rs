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

pub struct DeleteObjectTagging<'a, T: AsRef<str>,>(SubResource<'a, T>);

impl<'a, T: AsRef<str>,> DeleteObjectTagging<'a, T> {
    /// Create a new DeleteObjectTagging request with default parameters
    pub fn new(bucket: T, key: &'a str) -> Self {
        DeleteObjectTagging(SubResource {
            bucket,
            method: Method::DELETE,
            key: Some(key),
            params: vec![(QueryParameter::TAGGING, None)],
        })
    }

    pub fn version_id(mut self, version_id: &'a str) -> Self {
        self.0
            .params
            .push((QueryParameter::VERSION_ID, Some(version_id)));
        self
    }
}

impl<'a, T: AsRef<str>,> AwsRequest for DeleteObjectTagging<'a, T> {
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
            SubResource::<'a, T>::into_response(response).await?;

            Ok(())
        })
    }
}
