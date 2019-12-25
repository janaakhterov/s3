use crate::{auth_builder::AuthBuilder, buf_mut::BufMut, region::Region, signing_key::SigningKey};
use chrono::{offset::TimeZone, Utc};
use reqwest::{header::HeaderValue, Method, Request, Url};
use sha2::{Digest, Sha256};
use std::fmt::Display;

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

    pub fn into_request(
        self,
        url: Url,
        access_key: &str,
        signing_key: &SigningKey,
        region: Region,
    ) -> Result<Request, Box<dyn std::error::Error>> {
        // GetObject request do not have a payload; ever. So, computing one here
        // is a waste of time.
        let payload_hash = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";

        let datetime = Utc.ymd(2013, 05, 24).and_hms(0, 0, 0);
        let date = format!("{}", datetime.format("%Y%m%dT%H%M%SZ"));

        let url = url.join(&format!("{}/", self.bucket))?;
        let url = url.join(&self.name.as_ref())?;

        let mut request = Request::new(Method::GET, url.clone());

        let mut sig = AuthBuilder::new(
            request.headers_mut(),
            Method::GET.as_str(),
            region,
            datetime,
        );

        // Resource
        sig.set_resource(Some(&self.name));
        sig.set_query_params();

        sig.add_header(Headers::HOST, &url.domain().unwrap_or(""));

        if let Some(range) = self.range {
            sig.add_header(Headers::RANGE, range);
        }

        sig.add_header(Headers::X_AMZ_CONTENT_SHA256, &payload_hash);
        sig.add_header(Headers::X_AMZ_DATE, &date);

        if let Some(version_id) = self.version_id {
            sig.add_header(Headers::VERSION_ID, version_id);
        }

        sig.set_signed_headers();

        sig.set_payload(&payload_hash);

        sig.build(&access_key, &signing_key)?;

        println!("{:#?}", request);

        Ok(Request::new(
            Method::GET,
            Url::parse("https://google.com/").unwrap(),
        ))
    }
}

struct Headers;

impl Headers {
    const HOST: &'static str = "host";
    const IF_MATCH: &'static str = "if-match";
    const IF_MODIFIED_SINCE: &'static str = "if-modified-since";
    const IF_NONE_MATCHED: &'static str = "if-none-matched";
    const IF_UNMODIFIED_SINCE: &'static str = "if-unmodified-since";
    const PART_NUMBER: &'static str = "part-number";
    const RANGE: &'static str = "range";
    const REQUEST_PAYER: &'static str = "request-payer";
    const RESPONSE_CACHE_CONTROL: &'static str = "response-cache-control";
    const RESPONSE_CONTENT_DISPOSITION: &'static str = "response-content-disposition";
    const RESPONSE_CONTENT_ENCODING: &'static str = "response-content-encoding";
    const RESPONSE_CONTENT_LANGUAGE: &'static str = "responsee-content-language";
    const RESPONSE_CONTENT_TYPE: &'static str = "response-content-type";
    const RESPONSE_EXPIRES: &'static str = "response-expires";
    const SSE_CUSTOMER_ALGORITHM: &'static str = "sse-customer-algorithm";
    const SSE_CUSTOMER_KEY: &'static str = "sse-customer-key";
    const SSE_CUSTOMER_KEY_MD5: &'static str = "sse-customer-key-md5";
    const VERSION_ID: &'static str = "vesrion-id";
    const X_AMZ_CONTENT_SHA256: &'static str = "x-amz-content-sha256";
    const X_AMZ_DATE: &'static str = "x-amz-date";
}

// impl AmzRequest for GetObject {
// }
