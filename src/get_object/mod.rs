use crate::{
    AwsRequest,
    Error,
    Gmt,
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
use std::marker::PhantomData;

// mod optional;
mod response;

// pub(super) use optional::OptionalGetObject;
use response::FromGetObjectResponse;
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

pub struct GetObject<T: AsRef<str>, R: FromGetObjectResponse> {
    pub bucket: T,
    pub key: T,
    pub if_match: Option<T>,
    pub if_modified_since: Option<DateTime<Utc>>,
    pub if_none_match: Option<T>,
    pub if_unmodified_since: Option<DateTime<Utc>>,
    pub range: Option<String>,
    pub version_id: Option<T>,
    pub _phantom: PhantomData<R>,
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

impl<T: AsRef<str>> GetObject<T, GetObjectResponse> {
    pub fn new(bucket: T, key: T) -> GetObject<T, GetObjectResponse> {
        GetObject {
            bucket,
            key,
            if_match: None,
            if_modified_since: None,
            if_none_match: None,
            if_unmodified_since: None,
            range: None,
            version_id: None,
            _phantom: PhantomData,
        }
    }

    pub fn if_match(self, etag: T) -> GetObject<T, Option<GetObjectResponse>> {
        GetObject {
            bucket: self.bucket,
            key: self.key,
            if_match: Some(etag),
            if_modified_since: self.if_modified_since,
            if_none_match: self.if_none_match,
            if_unmodified_since: self.if_unmodified_since,
            range: self.range,
            version_id: self.version_id,
            _phantom: PhantomData,
        }
    }

    pub fn if_modified_since(
        self,
        since: DateTime<Utc>,
    ) -> GetObject<T, Option<GetObjectResponse>> {
        GetObject {
            bucket: self.bucket,
            key: self.key,
            if_match: self.if_match,
            if_modified_since: Some(since),
            if_none_match: self.if_none_match,
            if_unmodified_since: self.if_unmodified_since,
            range: self.range,
            version_id: self.version_id,
            _phantom: PhantomData,
        }
    }

    pub fn if_none_match(self, etag: T) -> GetObject<T, Option<GetObjectResponse>> {
        GetObject {
            bucket: self.bucket,
            key: self.key,
            if_match: self.if_match,
            if_modified_since: self.if_modified_since,
            if_none_match: Some(etag),
            if_unmodified_since: self.if_unmodified_since,
            range: self.range,
            version_id: self.version_id,
            _phantom: PhantomData,
        }
    }

    pub fn if_unmodified_since(
        self,
        since: DateTime<Utc>,
    ) -> GetObject<T, Option<GetObjectResponse>> {
        GetObject {
            bucket: self.bucket,
            key: self.key,
            if_match: self.if_match,
            if_modified_since: self.if_modified_since,
            if_none_match: self.if_none_match,
            if_unmodified_since: Some(since),
            range: self.range,
            version_id: self.version_id,
            _phantom: PhantomData,
        }
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

impl<T: AsRef<str>> AwsRequest for GetObject<T, GetObjectResponse> {
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
            .option_header(Headers::IF_MATCH, &self.if_match)?
            .option_header(
                Headers::IF_MODIFIED_SINCE,
                &self.if_modified_since.map(|since| since.to_gmt()),
            )?
            .option_header(Headers::IF_NONE_MATCH, &self.if_none_match)?
            .option_header(
                Headers::IF_UNMODIFIED_SINCE,
                &self.if_unmodified_since.map(|since| since.to_gmt()),
            )?
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

impl<T: AsRef<str>> AwsRequest for GetObject<T, Option<GetObjectResponse>> {
    type Response = Option<GetObjectResponse>;

    fn into_request<AR: AsRef<str>>(
        self,
        uri: Uri,
        access_key: AR,
        signing_key: &SigningKey,
        region: Region,
    ) -> Result<Request<HttpBody>, Error> {
        <GetObject<T, GetObjectResponse> as AwsRequest>::into_request(
            GetObject {
                bucket: self.bucket,
                key: self.key,
                if_match: self.if_match,
                if_modified_since: self.if_modified_since,
                if_none_match: self.if_none_match,
                if_unmodified_since: self.if_unmodified_since,
                range: self.range,
                version_id: self.version_id,
                _phantom: PhantomData,
            },
            uri,
            access_key,
            signing_key,
            region,
        )
    }

    fn into_response(
        response: Response<HttpBody>,
    ) -> BoxFuture<'static, Result<Self::Response, Error>> {
        Self::Response::from_response(response)
    }
}
