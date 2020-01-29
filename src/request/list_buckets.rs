use crate::{
    error,
    AwsRequest,
    AwsResponse,
    Error,
    Headers,
    Host,
    PayloadHash,
    Region,
    SignRequest,
    SigningKey,
};
use chrono::{
    DateTime,
    Utc,
};
use futures_core::future::BoxFuture;
use http::method::Method;
use hyper::{
    Body as HttpBody,
    Request,
    Response,
};
use serde::Deserialize;
use url::Url;

// ListBucket request Headers, this list *MUST* be in
// sorted order as it is used in the signing process
// of each request.
const HEADERS: [&str; 3] = [
    Headers::HOST,
    Headers::X_AMZ_CONTENT_SHA256,
    Headers::X_AMZ_DATE,
];

#[derive(Debug, Deserialize)]
pub(super) struct Owner {
    #[serde(rename = "ID")]
    pub(super) id: Option<String>,

    #[serde(rename = "DisplayName")]
    pub(super) display_name: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename = "ListAllMyBucketsResult")]
#[serde(rename_all = "PascalCase")]
struct ListBucketsResponse {
    owner: Owner,
    buckets: Buckets,
}

#[derive(Debug, Deserialize)]
struct Buckets {
    #[serde(rename = "Bucket")]
    buckets: Vec<Bucket>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Bucket {
    pub creation_date: DateTime<Utc>,
    pub name: String,
}

/// ListBucket request doesn't have any parameters.
/// Returns a list of buckets owned by the requestor.
pub struct ListBuckets;

impl AwsRequest for ListBuckets {
    type Response = Vec<Bucket>;

    fn into_request<T: AsRef<str>>(
        self,
        url: Url,
        access_key: T,
        signing_key: &SigningKey,
        region: Region,
    ) -> Result<Request<HttpBody>, Error> {
        let request = Request::builder()
            .method(Method::GET)
            .host(url, "", "", None)?
            .payload_hash(None)?
            .sign(&access_key.as_ref(), &signing_key, region.clone(), &HEADERS)?;

        Ok(request.body(HttpBody::empty()).map_err(error::Internal::from)?)
    }

    fn into_response(
        mut response: Response<HttpBody>,
    ) -> BoxFuture<'static, Result<Self::Response, Error>> {
        Box::pin(async move {
            let bytes = response.error().await?;
            let results = String::from_utf8_lossy(&bytes);
            let results: ListBucketsResponse = quick_xml::de::from_str(&results)
                        .map_err(error::Internal::from)?;

            Ok(results.buckets.buckets)
        })
    }
}
