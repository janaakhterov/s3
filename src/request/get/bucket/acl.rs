use crate::{
    AwsRequest,
    Error,
    QueryParameter,
    Permission,
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

#[derive(Debug, Deserialize)]
pub struct Grant {
    #[serde(rename = "Grantee")]
    grantee: Grantee,

    #[serde(rename = "Permission")]
    permission: Permission,
}

#[derive(Debug, Deserialize)]
pub struct Grantee {
    #[serde(rename = "DisplayName")]
    display_name: Option<String>,

    #[serde(rename = "EmailAddress")]
    email_address: Option<String>,

    #[serde(rename = "ID")]
    id: Option<String>,

    #[serde(rename = "Type")]
    r#type: String,

    #[serde(rename = "URI")]
    uri: Option<String>,
}

pub struct GetBucketAcl<'a, T: AsRef<str>,>(SubResource<'a, T>);

impl<'a, T: AsRef<str>,> GetBucketAcl<'a, T> {
    /// Create a new GetBucketAcl request with default parameters
    pub fn new(bucket: T) -> Self {
        GetBucketAcl(SubResource {
            bucket,
            method: Method::GET,
            key: None,
            params: vec![(QueryParameter::ACL, None)],
        })
    }
}

impl<'a, T: AsRef<str>,> AwsRequest for GetBucketAcl<'a, T> {
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
            let bytes = SubResource::<'a, T>::into_response(response).await?;
            let string = String::from_utf8_lossy(&bytes);

            let resp: BucketAcl = quick_xml::de::from_str(&string)?;

            Ok(resp)
        })
    }
}
