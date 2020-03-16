use crate::{
    error,
    Headers,
    Region,
    Result,
    SigningKey,
};
use chrono::Utc;
use http::{
    uri::{
        PathAndQuery,
        Uri,
    },
    HeaderValue,
};
use hyper::{
    Body as HttpBody,
    Method,
    Request as HttpRequest,
};
use sha2::{
    Digest,
    Sha256,
};
use std::{
    borrow::Cow,
    convert::TryFrom,
    ops::Deref,
};
use url::Url;

macro_rules! impl_sub_resource {
    ($name: ident => $output: ty) => {
        use crate::{
            error,
            AwsRequest,
            Error,
            QueryParameter,
            Region,
            SigningKey,
            SubResource,
        };
        use futures_core::future::BoxFuture;
        use hyper::{
            Body as HttpBody,
            Method,
            Request,
            Response,
        };
        use url::Url;

        pub struct $name<'a>(SubResource<'a>);

        impl<'a> AwsRequest for $name<'a> {
            type Response = $output;

            fn into_request<AR: AsRef<str>>(
                self,
                url: Url,
                access_key: AR,
                signing_key: &SigningKey,
                region: Region,
            ) -> Result<Request<HttpBody>, Error> {
                self.0.into_request(url, access_key, signing_key, region)
            }

            fn into_response(
                response: Response<HttpBody>,
            ) -> BoxFuture<'static, Result<Self::Response, Error>> {
                Box::pin(async move {
                    let bytes = SubResource::<'a>::into_response(response).await?;
                    let string = String::from_utf8_lossy(&bytes);

                    println!("-------------------------------------");
                    println!("{}", string);
                    println!("-------------------------------------");

                    let resp: $output =
                        quick_xml::de::from_str(&string).map_err(error::Internal::from)?;

                    Ok(resp)
                })
            }
        }
    };
}

pub mod create_bucket;
pub mod list_buckets;
// pub mod put_bucket_encryption;
pub mod put_object;

pub mod delete;
pub mod get;

pub(crate) mod sub_resource;

pub use create_bucket::*;
pub use list_buckets::*;
// pub use put_bucket_encryption::*;
pub use put_object::*;

pub use delete::*;
pub use get::*;

const NO_PAYLOAD_HASH: &str = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";

pub struct Request<'a> {
    pub bucket: Option<&'a str>,
    pub key: Option<&'a str>,
    pub method: Method,
    pub query: Vec<(&'static str, Option<&'a str>)>,
    pub body: Option<HttpBody>,
    pub region: Option<String>,
    pub headers: Vec<(&'static str, Cow<'a, str>)>,
    uri: Option<Uri>,
    hash: Option<Cow<'static, str>>,
}

impl<'a> Request<'a> {
    pub fn new(method: Method) -> Self {
        Self {
            method,
            bucket: None,
            key: None,
            query: Vec::new(),
            body: None,
            region: None,
            headers: Vec::new(),
            uri: None,
            hash: None,
        }
    }

    pub fn bucket(mut self, bucket: &'a str) -> Self {
        self.bucket = Some(bucket);
        self
    }

    pub fn region(mut self, region: Region) -> Self {
        let region: String = region.into();
        self.region = Some(region);
        self
    }

    pub fn key(mut self, key: &'a str) -> Self {
        self.key = Some(key);
        self
    }

    pub fn query(mut self, key: &'static str, value: Option<&'a str>) -> Self {
        match self.query.binary_search_by(|&(k, _)| k.cmp(key)) {
            Ok(_) => {}
            Err(index) => self.query.insert(index, (key, value)),
        }
        self
    }

    pub fn header<T: Into<Cow<'a, str>>>(mut self, key: &'static str, value: Option<T>) -> Self {
        if let Some(value) = value {
            match self.headers.binary_search_by(|&(k, _)| k.cmp(key)) {
                Ok(_) => {}
                Err(index) => self.headers.insert(index, (key, value.into())),
            }
        }
        self
    }

    pub fn host(mut self, url: Url) -> Result<Self> {
        let url_domain = url.domain().unwrap();

        let domain = if let Some(bucket) = self.bucket {
            format!("{}.{}", bucket, url.domain().unwrap())
        } else {
            url_domain.to_owned()
        };

        let uri = format!(
            "{}://{}:{}/{}",
            url.scheme(),
            domain,
            url.port().map(|v| v.to_string()).unwrap_or("".to_owned()),
            self.key.unwrap_or("")
        );

        let domain = if let Some(ref region) = self.region {
            format!("{}.{}.{}", self.bucket.unwrap_or(""), region, url_domain)
        } else {
            domain
        };

        self.uri = Some(Uri::try_from(&uri).map_err(error::Internal::from)?);
        self = self.header(Headers::HOST, Some(domain));
        Ok(self)
    }

    pub fn body(mut self, bytes: Vec<u8>) -> Self {
        let mut hasher = Sha256::new();
        hasher.input(&bytes);

        let payload_hash = hex::encode(hasher.result().as_slice());

        self = self.header(Headers::X_AMZ_CONTENT_SHA256, Some(payload_hash.clone()));
        self.hash = Some(Cow::Owned(payload_hash));

        let content_md5 = base64::encode(&*md5::compute(&bytes));
        self = self.header(Headers::CONTENT_MD5, Some(content_md5));

        self.body = Some(HttpBody::from(bytes));

        self
    }

    fn build(
        mut self,
        access_key: &'a str,
        signing_key: &'a SigningKey,
    ) -> Result<HttpRequest<HttpBody>> {
        let mut canonical: Vec<u8> = Vec::new();
        let mut signed: Vec<&str> = Vec::new();

        let mut query = Vec::new();
        for (key, value) in self.query.iter() {
            query.push(if let Some(value) = value.as_ref() {
                format!("{}={}", key, value)
            } else {
                format!("{}", key)
            });
        }
        let query = if self.query.len() > 0 {
            format!("?{}", query.join(","))
        } else {
            query.join(",")
        };

        let uri = self
            .uri
            .clone()
            .ok_or(error::Internal::Message("URI body not called".to_string()))?;

        let mut parts = uri.into_parts();
        parts.path_and_query = Some(
            PathAndQuery::try_from(
                format!("/{}{}", self.key.unwrap_or(""), query.as_str()).as_str(),
            )
            .map_err(error::Internal::from)?,
        );

        let uri = Uri::from_parts(parts).map_err(error::Internal::from)?;

        let mut request = HttpRequest::builder();

        // Request Method
        canonical.extend_from_slice(&self.method.as_str().as_bytes());
        canonical.push(b'\n');

        request = request.method(self.method.clone());

        // Request Uri
        canonical.extend_from_slice(&uri.path().as_bytes());
        canonical.push(b'\n');

        request = request.uri(uri);

        // Request Query Parameters
        canonical.extend_from_slice(&query.as_bytes());
        canonical.push(b'\n');

        // Request Query Parameters ready set above

        // The `X_AMZ_CONTENT_SHA256` header is *always* requried. If `Request::body()`
        // was not called then we simply set the header to the NO_PAYLOAD_HASH constant
        if self.hash.is_none() {
            self = self.header(Headers::X_AMZ_CONTENT_SHA256, Some(NO_PAYLOAD_HASH));
        }
        // All requets should have date
        // Formatting date in rfc1123 was rejected by minio even though it says to use that format
        // instead using format from aws examples YYYYMMDDTHHMMSSZ
        let date = Utc::now();
        self = self.header(
            Headers::X_AMZ_DATE,
            Some(format!("{}", date.format("%Y%m%dT%H%M%SZ"))),
        );

        // Request Headers
        for (header, value) in self.headers.iter() {
            request = request.header(
                *header,
                HeaderValue::from_str(value.deref()).map_err(error::Internal::from)?,
            );
            canonical.extend_from_slice(&header.as_bytes());
            canonical.push(b':');
            canonical.extend_from_slice(&value.as_bytes());
            canonical.push(b'\n');

            // Signed contains an ordered list of headers that are used to sign the request
            signed.push(header)
        }

        // End of Headers
        canonical.push(b'\n');

        // Signed Headers
        let signed = signed.join(";");
        canonical.extend_from_slice(&signed.as_bytes());
        canonical.push(b'\n');

        let hash: &Cow<'static, str> = if let Some(ref hash) = self.hash {
            hash
        } else {
            &Cow::Borrowed(NO_PAYLOAD_HASH)
        };

        canonical.extend_from_slice(&hash.deref().as_bytes());

        println!("{}", String::from_utf8_lossy(&canonical));

        let mut hasher = Sha256::new();
        hasher.input(canonical);

        let hash = hex::encode(hasher.result().as_slice());

        let scope = format!(
            "{date}/{region}/s3/aws4_request",
            date = date.format("%Y%m%d"),
            region = self.region.ok_or(error::Internal::Message(
                "Region was not set before signing".to_string()
            ))?
        );

        let string_to_sign = format!(
            "AWS4-HMAC-SHA256\n{timestamp}\n{scope}\n{hash}",
            timestamp = date.format("%Y%m%dT%H%M%SZ"),
            scope = scope,
            hash = hash
        );

        let sig = signing_key.sign(string_to_sign);

        let auth = format!(
            "AWS4-HMAC-SHA256 Credential={access_key}/{scope},SignedHeaders={signed_headers},Signature={signature}",
            access_key = access_key,
            scope = scope,
            signed_headers = signed,
            signature = sig
        );

        request = request.header(
            Headers::AUTHORIZATION,
            HeaderValue::from_str(&auth).map_err(error::Internal::from)?,
        );

        if let Some(body) = self.body {
            Ok(request.body(body).map_err(error::Internal::from)?)
        } else {
            Ok(request
                .body(HttpBody::empty())
                .map_err(error::Internal::from)?)
        }
    }
}
