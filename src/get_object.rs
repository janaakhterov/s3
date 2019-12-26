use crate::{Error, AmzRequest, AuthBuilder, Headers, Region, SigningKey};
use chrono::Utc;
use reqwest::{Method, Request, Url};
use std::fmt::Display;
use futures_core::future::BoxFuture;

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

        let mut sig = AuthBuilder::new(
            request.headers_mut(),
            Method::GET.as_str(),
            region,
            datetime,
        );

        // Resource
        sig.set_resource(Some(&resource));
        sig.set_query_params();

        let host = url.host_str().unwrap();

        sig.add_header(Headers::HOST, &host)?;

        if let Some(range) = self.range {
            sig.add_header(Headers::RANGE, range)?;
        }

        sig.add_header(Headers::X_AMZ_CONTENT_SHA256, &payload_hash)?;
        sig.add_header(Headers::X_AMZ_DATE, &date)?;

        if let Some(version_id) = self.version_id {
            sig.add_header(Headers::VERSION_ID, version_id)?;
        }

        sig.set_signed_headers();

        sig.set_payload(&payload_hash);

        sig.build(&access_key, &signing_key)?;

        Ok(request)
    }

    fn into_response(response: reqwest::Response) -> BoxFuture<'static, Result<Self::Response, Error>> {
        Box::pin(async move {
            Ok(response.bytes().await?)
        })
    }
}
