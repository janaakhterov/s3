use crate::{Error, AmzRequest, Headers, Region, SigningKey, sign_request};
use chrono::Utc;
use reqwest::{Method, Request, Url, header::HeaderValue};
use std::fmt::Display;
use futures_core::future::BoxFuture;

pub const HEADERS: [&'static str; 20] = [
    Headers::HOST,
    Headers::IF_MATCH,
    Headers::IF_MODIFIED_SINCE,
    Headers::IF_NONE_MATCHED,
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

pub struct GetObject<T: AsRef<str> + Display> {
    pub bucket: T,
    pub name: T,
    // pub if_match: Option<T>,
    // pub if_modified_since: Option<T>,
    // pub if_none_match: Option<T>,
    // pub if_unmodified_since: Option<T>,
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

impl<T: AsRef<str> + Display> GetObject<T> {
    pub fn new(bucket: T, name: T) -> Self {
        GetObject {
            bucket,
            name,
            range: None,
            version_id: None,
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

impl<T: AsRef<str> + Display> AmzRequest for GetObject<T> {
    type Response = bytes::Bytes;

    fn into_request<S: AsRef<str> + Display>(
        self,
        url: Url,
        access_key: S,
        signing_key: &SigningKey,
        region: Region,
    ) -> Result<Request, Error> {
        // GetObject request do not have a payload; ever. So, computing one here
        // is a waste of time.
        let payload_hash = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";

        let datetime = Utc::now();
        let date = format!("{}", datetime.format("%Y%m%dT%H%M%SZ"));

        let resource = format!("{}/{}", self.bucket, self.name.as_ref());
        let url = url.join(&resource)?;

        let mut request = Request::new(Method::GET, url.clone());
        let headers = request.headers_mut();

        let host = url.host_str().ok_or(Error::HostStrUnset)?;

        headers.insert(Headers::HOST, HeaderValue::from_str(&host)?);

        if let Some(range) = self.range {
            headers.insert(Headers::RANGE, HeaderValue::from_str(&range)?);
        }

        headers.insert(Headers::X_AMZ_CONTENT_SHA256, HeaderValue::from_str(&payload_hash)?);
        headers.insert(Headers::X_AMZ_DATE, HeaderValue::from_str(&date)?);

        if let Some(version_id) = self.version_id {
            headers.insert(Headers::VERSION_ID, HeaderValue::from_str(version_id.as_ref())?);
        }

        sign_request(
            &mut request, 
            &access_key, 
            &signing_key, 
            region.clone(), 
            &HEADERS
        )?;

        Ok(request)
    }

    fn into_response(response: reqwest::Response) -> BoxFuture<'static, Result<Self::Response, Error>> {
        Box::pin(async move {
            Ok(response.bytes().await?)
        })
    }
}
