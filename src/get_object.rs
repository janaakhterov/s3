use crate::{sign_request, Error, Gmt, Headers, Region, S3Request, SigningKey, StorageClass};
use chrono::{DateTime, Utc};
use futures_core::future::BoxFuture;
use http::uri::{PathAndQuery, Uri};
use http_body::Body;
use hyper::{header::HeaderValue, Body as HttpBody, Method, Request, Response};
use std::convert::TryFrom;
use std::str::FromStr;

pub const HEADERS: [&'static str; 20] = [
    Headers::HOST,
    Headers::IF_MATCH,
    Headers::IF_MODIFIED_SINCE,
    Headers::IF_NONE_MATCH,
    Headers::IF_UNMODIFIED_SINCE,
    Headers::PART_NUMBER,
    Headers::RANGE,
    Headers::REQUEST_PAYER,
    Headers::RESPONSE_CACHE_CONTROL,
    Headers::RESPONSE_CONTENT_DISPOSITION,
    Headers::RESPONSE_CONTENT_ENCODING,
    Headers::RESPONSE_CONTENT_LANGUAGE,
    Headers::RESPONSE_CONTENT_TYPE,
    Headers::RESPONSE_EXPIRES,
    Headers::SSE_CUSTOMER_ALGORITHM,
    Headers::SSE_CUSTOMER_KEY,
    Headers::SSE_CUSTOMER_KEY_MD5,
    Headers::VERSION_ID,
    Headers::X_AMZ_CONTENT_SHA256,
    Headers::X_AMZ_DATE,
];

pub struct GetObject<T: AsRef<str>> {
    pub bucket: T,
    pub name: T,
    pub if_match: Option<T>,
    pub if_modified_since: Option<DateTime<Utc>>,
    pub if_none_match: Option<T>,
    pub if_unmodified_since: Option<DateTime<Utc>>,
    // pub part_number: Option<u64>,
    pub range: Option<String>,
    // pub request_payer: Option<T>,
    // pub response_cache_control: Option<T>,
    // pub response_content_disposition: Option<T>,
    // pub response_content_encoding: Option<T>,
    // pub response_content_language: Option<T>,
    // pub response_content_type: Option<T>,
    // pub response_expires: Option<T>,
    // pub sse_customer_algorithm: Option<T>,
    // pub sse_customer_key: Option<T>,
    // pub sse_customer_key_md5: Option<T>,
    pub version_id: Option<T>,
}

impl<T: AsRef<str>> GetObject<T> {
    pub fn new(bucket: T, name: T) -> Self {
        GetObject {
            bucket,
            name,
            if_match: None,
            if_modified_since: None,
            if_none_match: None,
            if_unmodified_since: None,
            range: None,
            version_id: None,
        }
    }

    pub fn if_match(mut self, etag: T) -> Self {
        self.if_match = Some(etag);
        self
    }

    pub fn if_modified_since(mut self, since: DateTime<Utc>) -> Self {
        self.if_modified_since = Some(since);
        self
    }

    pub fn if_none_match(mut self, etag: T) -> Self {
        self.if_none_match = Some(etag);
        self
    }

    pub fn if_unmodified_since(mut self, since: DateTime<Utc>) -> Self {
        self.if_unmodified_since = Some(since);
        self
    }

    pub fn range(mut self, start: u64, end: u64) -> Self {
        self.range = Some(format!("bytes={}-{}", start, end));
        self
    }

    pub fn version_id(mut self, version_id: T) -> Self {
        self.version_id = Some(version_id);
        self
    }
}

#[derive(Debug)]
pub struct GetObjectResponse {
    pub last_modified: DateTime<Utc>,
    pub etag: String,
    pub version_id: Option<String>,
    pub expires: Option<DateTime<Utc>>,
    pub storage_class: StorageClass,
    pub parts_count: Option<u64>,
    pub body: Vec<u8>,
}

impl GetObjectResponse {
    pub fn as_str(&self) -> std::borrow::Cow<str> {
        String::from_utf8_lossy(&self.body)
    }
}

impl<T: AsRef<str>> S3Request for GetObject<T> {
    type Response = GetObjectResponse;

    fn into_request<AR: AsRef<str>>(
        self,
        uri: Uri,
        access_key: AR,
        signing_key: &SigningKey,
        region: Region,
    ) -> Result<Request<HttpBody>, Error> {
        // GetObject request do not have a payload; ever. So, computing one here
        // is a waste of time.
        let payload_hash = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";

        let datetime = Utc::now();
        // Formatting date in rfc1123 was rejected by minio even though it says to use that format
        // instead using format from aws examples YYYYMMDDTHHMMSSZ
        let date = &format!("{}", datetime.format("%Y%m%dT%H%M%SZ"));

        let resource = PathAndQuery::try_from(
            format!("/{}/{}", self.bucket.as_ref(), self.name.as_ref()).as_str(),
        )?;

        let parts = uri.clone().into_parts();

        let mut uri = Uri::builder();

        if let Some(scheme) = parts.scheme {
            uri = uri.scheme(scheme)
        }

        if let Some(authority) = parts.authority {
            uri = uri.authority(authority)
        }

        let uri = uri.path_and_query(resource).build()?;

        let host = uri.host().ok_or(Error::HostStrUnset)?.to_owned();

        let mut request = Request::builder().method(Method::GET).uri(uri);

        request = request.header(Headers::HOST, HeaderValue::from_str(&host)?);

        if let Some(if_match) = self.if_match {
            request = request.header(
                Headers::IF_MATCH,
                HeaderValue::from_str(&if_match.as_ref())?,
            );
        }

        if let Some(if_modified_since) = self.if_modified_since {
            request = request.header(
                Headers::IF_MODIFIED_SINCE,
                HeaderValue::from_str(&if_modified_since.to_gmt())?,
                // HeaderValue::from_str(&if_modified_since.to_rfc3339())?,
            );
        }

        if let Some(if_none_match) = self.if_none_match {
            request = request.header(
                Headers::IF_NONE_MATCH,
                HeaderValue::from_str(&if_none_match.as_ref())?,
            );
        }

        if let Some(if_unmodified_since) = self.if_unmodified_since {
            request = request.header(
                Headers::IF_UNMODIFIED_SINCE,
                HeaderValue::from_str(&if_unmodified_since.to_gmt())?,
            );
        }

        if let Some(range) = self.range {
            request = request.header(Headers::RANGE, HeaderValue::from_str(&range)?);
        }

        request = request.header(
            Headers::X_AMZ_CONTENT_SHA256,
            HeaderValue::from_str(&payload_hash)?,
        );
        request = request.header(Headers::X_AMZ_DATE, HeaderValue::from_str(&date)?);

        if let Some(version_id) = self.version_id {
            request = request.header(
                Headers::VERSION_ID,
                HeaderValue::from_str(version_id.as_ref())?,
            );
        }

        let request = sign_request(
            request,
            &access_key.as_ref(),
            &signing_key,
            region.clone(),
            &HEADERS,
        )?;

        println!("{:#?}", request);

        Ok(request.body(HttpBody::empty())?)
    }

    fn into_response(
        mut response: Response<HttpBody>,
    ) -> BoxFuture<'static, Result<Self::Response, Error>> {
        Box::pin(async move {
            let last_modified = response
                .headers()
                .get(Headers::LAST_MODIFIED)
                .map(HeaderValue::to_str)
                .transpose()?
                .map(DateTime::from_gmt)
                .transpose()?
                .ok_or(Error::LastModifiedNotPresentOnGetResponse)?;

            let etag: String = response
                .headers()
                .get(Headers::ETAG)
                .ok_or(Error::NoEtagInRespoinse)?
                .to_str()?
                .to_owned();

            let version_id: Option<String> = response
                .headers()
                .get(Headers::X_AMZ_VERSION_ID)
                .map(HeaderValue::to_str)
                .transpose()?
                .map(str::to_owned);

            let expires = response
                .headers()
                .get(Headers::EXPIRES)
                .map(HeaderValue::to_str)
                .transpose()?
                .map(DateTime::from_gmt)
                .transpose()?;

            let storage_class: StorageClass =
                if let Some(header) = response.headers().get(Headers::X_AMZ_STORAGE_CLASS) {
                    StorageClass::from_str(header.to_str()?)?
                } else {
                    StorageClass::Standard
                };

            let parts_count: Option<u64> = response
                .headers()
                .get(Headers::PARTS_COUNT)
                .map(HeaderValue::to_str)
                .transpose()?
                .map(u64::from_str)
                .transpose()?;

            let mut bytes: Vec<u8> = Vec::new();

            while let Some(next) = response.data().await {
                let chunk = next?;
                bytes.extend_from_slice(&chunk);
            }

            Ok(GetObjectResponse {
                last_modified,
                etag,
                version_id,
                storage_class,
                expires,
                parts_count,
                body: bytes,
            })
        })
    }
}
