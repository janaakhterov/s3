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
use serde::Deserialize;
use crate::request::list_buckets::Owner;
use crate::types::Grant;
use url::Url;

#[derive(Debug, Deserialize)]
pub struct BucketAcl {
    #[serde(rename = "Owner")]
    owner: Owner,

    #[serde(rename = "AccessControlList")]
    list: AccessControlList
}

#[derive(Debug, Deserialize)]
pub struct AccessControlList {
    #[serde(rename = "Grant")]
    grants: Vec<Grant>,
}

pub struct GetBucketAcl<'a>(SubResource<'a>);

impl<'a> GetBucketAcl<'a> {
    /// Create a new GetBucketAcl request with default parameters
    pub fn new(bucket: &'a str) -> Self {
        GetBucketAcl(SubResource {
            bucket,
            method: Method::GET,
            key: None,
            params: vec![(QueryParameter::ACL, None)],
        })
    }
}

impl<'a> AwsRequest for GetBucketAcl<'a> {
    type Response = BucketAcl;

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

            let resp: BucketAcl = quick_xml::de::from_str(&string)
                        .map_err(error::Internal::from)?;

            Ok(resp)
        })
    }
}
