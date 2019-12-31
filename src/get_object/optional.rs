use super::{
    GetObject,
    GetObjectResponse,
    HEADERS,
};
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
    StatusCode,
};

pub struct OptionalGetObject<T: AsRef<str>> {
    pub(super) bucket: T,
    pub(super) key: T,
    pub(super) if_match: Option<T>,
    pub(super) if_modified_since: Option<DateTime<Utc>>,
    pub(super) if_none_match: Option<T>,
    pub(super) if_unmodified_since: Option<DateTime<Utc>>,
    pub(super) range: Option<String>,
    pub(super) version_id: Option<T>,
}

impl<T: AsRef<str>> OptionalGetObject<T> {
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

impl<T: AsRef<str>> From<GetObject<T>> for OptionalGetObject<T> {
    fn from(get: GetObject<T>) -> Self {
        OptionalGetObject {
            bucket: get.bucket,
            key: get.key,
            version_id: get.version_id,
            range: get.range,
            if_match: None,
            if_modified_since: None,
            if_none_match: None,
            if_unmodified_since: None,
        }
    }
}

impl<T: AsRef<str>> AwsRequest for OptionalGetObject<T> {
    type Response = Option<GetObjectResponse>;

    fn into_request<AR: AsRef<str>>(
        self,
        uri: Uri,
        access_key: AR,
        signing_key: &SigningKey,
        region: Region,
    ) -> Result<Request<HttpBody>, Error> {
        // OptionalGetObject request do not have a payload; ever. So, computing one here
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
        Box::pin(async move {
            if response.status() == StatusCode::NOT_MODIFIED {
                return Ok(None);
            }

            Ok(Some(GetObjectResponse::from_response(response).await?))
        })
    }
}
