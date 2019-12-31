use crate::{
    AwsRequest,
    Error,
    Headers,
    Host,
    OptionHeader,
    Region,
    SignRequest,
    SigningKey,
};
use chrono::{
    DateTime,
    Utc,
};
use futures_core::future::BoxFuture;
use http::uri::Uri;
use hyper::{
    header::HeaderValue,
    Body as HttpBody,
    Method,
    Request,
    Response,
};

mod optional;
mod response;

pub(super) use optional::OptionalGetObject;
pub(super) use response::GetObjectResponse;

pub(super) const HEADERS: [&'static str; 20] = [
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
    pub(super) bucket: T,
    pub(super) key: T,
    pub(super) range: Option<String>,
    pub(super) version_id: Option<T>,
}

// TODO:
// pub part_number: Option<u64>,
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

impl<T: AsRef<str>> GetObject<T> {
    pub fn new(bucket: T, key: T) -> Self {
        GetObject {
            bucket,
            key,
            range: None,
            version_id: None,
        }
    }

    pub fn if_match(self, etag: T) -> OptionalGetObject<T> {
        let optional = OptionalGetObject::from(self);
        optional.if_match(etag)
    }

    pub fn if_modified_since(self, since: DateTime<Utc>) -> OptionalGetObject<T> {
        let optional = OptionalGetObject::from(self);
        optional.if_modified_since(since)
    }

    pub fn if_none_match(self, etag: T) -> OptionalGetObject<T> {
        let optional = OptionalGetObject::from(self);
        optional.if_none_match(etag)
    }

    pub fn if_unmodified_since(self, since: DateTime<Utc>) -> OptionalGetObject<T> {
        let optional = OptionalGetObject::from(self);
        optional.if_unmodified_since(since)
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

impl<T: AsRef<str>> AwsRequest for GetObject<T> {
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

        let request = Request::builder()
            .method(Method::GET)
            .host(uri.clone(), self.bucket, self.key)?
            .option_header(Headers::RANGE, &self.range)?
            .option_header(Headers::VERSION_ID, &self.version_id)?
            .header(
                Headers::X_AMZ_CONTENT_SHA256,
                HeaderValue::from_str(&payload_hash)?,
            )
            .header(Headers::X_AMZ_DATE, HeaderValue::from_str(&date)?)
            .sign(&access_key.as_ref(), &signing_key, region.clone(), &HEADERS)?;

        println!("{:#?}", request);

        Ok(request.body(HttpBody::empty())?)
    }

    fn into_response(
        response: Response<HttpBody>,
    ) -> BoxFuture<'static, Result<Self::Response, Error>> {
        GetObjectResponse::from_response(response)
    }
}
