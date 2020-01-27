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
use serde::Deserialize;
use url::Url;

#[derive(Debug, Deserialize)]
pub struct GetBucketAccelerateConfigOutput {
    #[serde(rename = "Status")]
    status: String
}

pub struct GetBucketAccelerateConfig<'a, T: AsRef<str>,>(SubResource<'a, T>);

impl<'a, T: AsRef<str>,> GetBucketAccelerateConfig<'a, T> {
    /// Create a new GetBucketAccelerateConfig request with default parameters
    pub fn new(bucket: T) -> Self {
        GetBucketAccelerateConfig(SubResource {
            bucket,
            method: Method::GET,
            key: None,
            params: vec![(QueryParameter::ACCELERATE, None)],
        })
    }
}

impl<'a, T: AsRef<str>,> AwsRequest for GetBucketAccelerateConfig<'a, T> {
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
            let bytes = SubResource::<'a, T>::into_response(response).await?;
            let string = String::from_utf8_lossy(&bytes);

            let resp: GetBucketAccelerateConfigOutput = quick_xml::de::from_str(&string)?;

            println!("{:#?}", resp);

            Ok(())
        })
    }
}
