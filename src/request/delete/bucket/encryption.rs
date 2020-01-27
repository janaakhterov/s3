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

pub struct DeleteBucketEncryption<'a, T: AsRef<str>,>(SubResource<'a, T>);

impl<'a, T: AsRef<str>,> DeleteBucketEncryption<'a, T> {
    /// Create a new DeleteBucketEncryption request with default parameters
    pub fn new(bucket: T) -> Self {
        DeleteBucketEncryption(SubResource {
            bucket,
            method: Method::DELETE,
            key: None,
            params: vec![(QueryParameter::ENCRYPTION, None)],
        })
    }
}

impl<'a, T: AsRef<str>,> AwsRequest for DeleteBucketEncryption<'a, T> {
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
