use crate::{
    AwsRequest,
    AwsResponse,
    Error,
    Headers,
    Region,
    SignRequest,
    SigningKey,
};
use chrono::{
    DateTime,
    Utc,
};
use futures_core::future::BoxFuture;
use http::{
    header::HeaderValue,
    method::Method,
    uri::Uri,
};
use http_body::Body;
use hyper::{
    Body as HttpBody,
    Request,
    Response,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Owner {
    #[serde(rename = "ID")]
    pub id: String,

    #[serde(rename = "DisplayName")]
    pub display_name: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename = "ListAllMyBucketsResult")]
#[serde(rename_all = "PascalCase")]
pub struct ListBucketsResponse {
    owner: Owner,
    buckets: Buckets,
}

#[derive(Debug, Deserialize)]
pub struct Buckets {
    #[serde(rename = "Bucket")]
    buckets: Vec<Bucket>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Bucket {
    pub creation_date: DateTime<Utc>,
    pub name: String,
}

pub const HEADERS: [&'static str; 3] = [
    Headers::HOST,
    Headers::X_AMZ_CONTENT_SHA256,
    Headers::X_AMZ_DATE,
];

pub struct ListBuckets;

impl AwsRequest for ListBuckets {
    type Response = Vec<Bucket>;

    fn into_request<AR: AsRef<str>>(
        self,
        uri: Uri,
        access_key: AR,
        signing_key: &SigningKey,
        region: Region,
    ) -> Result<Request<HttpBody>, Error> {
        let payload_hash = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";

        let date = format!("{}", Utc::now().format("%Y%m%dT%H%M%SZ"));

        println!("host: {}", uri.host().unwrap_or(""));

        let request = Request::builder()
            .method(Method::GET)
            .uri(uri.clone())
            .header(
                Headers::HOST,
                HeaderValue::from_str(uri.host().unwrap_or(""))?,
            )
            .header(
                Headers::X_AMZ_CONTENT_SHA256,
                HeaderValue::from_str(&payload_hash)?,
            )
            .header(Headers::X_AMZ_DATE, HeaderValue::from_str(&date)?)
            .sign(&access_key.as_ref(), &signing_key, region.clone(), &HEADERS)?;

        println!("{:#?}", request);

        Ok(request.body(HttpBody::from(HttpBody::empty()))?)
    }

    fn into_response(
        mut response: Response<HttpBody>,
    ) -> BoxFuture<'static, Result<Self::Response, Error>> {
        Box::pin(async move {
            response.error().await?;

            let mut bytes: Vec<u8> = Vec::new();

            while let Some(next) = response.data().await {
                let chunk = next?;
                bytes.extend_from_slice(&chunk);
            }

            let results = String::from_utf8_lossy(&bytes);
            let results: ListBucketsResponse = quick_xml::de::from_str(&results)?;

            Ok(results.buckets.buckets)
        })
    }
}
