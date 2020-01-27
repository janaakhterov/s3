use crate::{
    AwsRequest,
    AwsResponse,
    Error,
    Gmt,
    Headers,
    Host,
    OptionalHeader,
    PayloadHash,
    Region,
    SignRequest,
    SigningKey,
    StorageClass,
};
use chrono::{
    DateTime,
    Utc,
};
use futures_core::future::BoxFuture;
use hyper::{
    Body as HttpBody,
    Method,
    Request,
    Response,
    StatusCode,
};
use std::{
    borrow::Cow,
    marker::PhantomData,
    ops::{
        Deref,
        DerefMut,
    },
};
use url::Url;

// GetObject request Headers, this list *MUST* be in
// sorted order as it is used in the signing process
// of each request.
pub(super) const HEADERS: [&str; 20] = [
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

// Reason for `R: FromGetObjectResponse` is because the response
// of the get request will become optional if any of the `if_*`
// options are set. Otherwise, the request will always return a
// value or an error.
pub struct GetObject<T: AsRef<str>, R: FromGetObjectResponse> {
    /// The bucket name containing the object.
    pub bucket: T,
    /// Key of the object to get.
    pub key: T,
    if_match: Option<T>,
    if_modified_since: Option<DateTime<Utc>>,
    if_none_match: Option<T>,
    if_unmodified_since: Option<DateTime<Utc>>,
    range: Option<String>,
    version_id: Option<T>,
    _phantom: PhantomData<R>,
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
    /// Create a new GetObject request with default parameters and non-optional
    /// response type.
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

    /// Return the object only if its entity tag (ETag) is the same as the one specified, otherwise return a 412 (precondition failed).
    /// **Note:** This changes the response type of from `GetObjectResponse` to `Option<GetObjectResponse>`
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

    /// Return the object only if it has been modified since the specified time, otherwise return a 304 (not modified).
    /// **Note:** This changes the response type of from `GetObjectResponse` to `Option<GetObjectResponse>`
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

    /// Return the object only if its entity tag (ETag) is different from the one specified, otherwise return a 304 (not modified).
    /// **Note:** This changes the response type of from `GetObjectResponse` to `Option<GetObjectResponse>`
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

    /// Return the object only if it has not been modified since the specified time, otherwise return a 412 (precondition failed).
    /// **Note:** This changes the response type of from `GetObjectResponse` to `Option<GetObjectResponse>`
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

    /// Downloads the specified range bytes of an object.
    pub fn range(mut self, start: u64, end: u64) -> Self {
        self.range = Some(format!("bytes={}-{}", start, end));
        self
    }

    /// VersionId used to reference a specific version of the object.
    pub fn version_id(mut self, version_id: T) -> Self {
        self.version_id = Some(version_id);
        self
    }
}

impl<T: AsRef<str>> AwsRequest for GetObject<T, GetObjectResponse> {
    type Response = GetObjectResponse;

    fn into_request<AR: AsRef<str>>(
        self,
        url: Url,
        access_key: AR,
        signing_key: &SigningKey,
        region: Region,
    ) -> Result<Request<HttpBody>, Error> {
        let request = Request::builder()
            .method(Method::GET)
            .host(url, self.bucket, self.key, None)?
            .optional_header(Headers::IF_MATCH, &self.if_match)?
            .optional_header(
                Headers::IF_MODIFIED_SINCE,
                &self.if_modified_since.map(|since| since.to_gmt()),
            )?
            .optional_header(Headers::IF_NONE_MATCH, &self.if_none_match)?
            .optional_header(
                Headers::IF_UNMODIFIED_SINCE,
                &self.if_unmodified_since.map(|since| since.to_gmt()),
            )?
            .optional_header(Headers::RANGE, &self.range)?
            .optional_header(Headers::VERSION_ID, &self.version_id)?
            .payload_hash(None)?
            .sign(&access_key.as_ref(), &signing_key, region.clone(), &HEADERS)?;

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
        url: Url,
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
            url,
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

pub trait FromGetObjectResponse {
    fn from_response(response: Response<HttpBody>) -> BoxFuture<'static, Result<Self, Error>>
    where
        Self: Sized;
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

impl Deref for GetObjectResponse {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.body
    }
}

impl DerefMut for GetObjectResponse {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.body
    }
}

impl GetObjectResponse {
    pub fn as_str(&self) -> Cow<str> {
        String::from_utf8_lossy(&self.body)
    }
}

impl FromGetObjectResponse for GetObjectResponse {
    fn from_response(mut response: Response<HttpBody>) -> BoxFuture<'static, Result<Self, Error>> {
        Box::pin(async move {
            let bytes = response.error().await?;

            let last_modified = response.last_modified()?;
            let etag = response.etag()?;
            let version_id = response.version_id()?;
            let expires = response.expires()?;
            let storage_class = response.storage_class()?;
            let parts_count = response.parts_count()?;

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

impl FromGetObjectResponse for Option<GetObjectResponse> {
    fn from_response(response: Response<HttpBody>) -> BoxFuture<'static, Result<Self, Error>> {
        Box::pin(async move {
            if response.status() == StatusCode::NOT_MODIFIED {
                return Ok(None);
            }

            Ok(Some(
                <GetObjectResponse as FromGetObjectResponse>::from_response(response).await?,
            ))
        })
    }
}
