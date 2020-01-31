use crate::{
    error,
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
use crate::types::BucketLocation;

pub struct GetBucketLocation<'a>(SubResource<'a>);

impl<'a> GetBucketLocation<'a> {
    /// Create a new GetBucketLocation request with default parameters
    pub fn new(bucket: &'a str) -> Self {
        GetBucketLocation(SubResource {
            bucket,
            method: Method::GET,
            key: None,
            params: vec![(QueryParameter::LOCATION, None)],
        })
    }
}

impl<'a> AwsRequest for GetBucketLocation<'a> {
    type Response = BucketLocation;

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

            let resp: BucketLocation = quick_xml::de::from_str(&string)
                        .map_err(error::Internal::from)?;

            Ok(resp)
        })
    }
}
