use crate::{sign_request, Error, Gmt, Headers, Region, S3Request, SigningKey};
use chrono::{DateTime, Utc};
use futures_core::future::BoxFuture;
use http::{
    header::HeaderValue,
    method::Method,
    uri::{PathAndQuery, Uri},
};
use hyper::{Body as HttpBody, Request, Response};
use sha2::{Digest, Sha256};
use std::convert::TryFrom;

pub const HEADERS: [&'static str; 5] = [
    Headers::CONTENT_MD5,
    Headers::EXPIRES,
    Headers::HOST,
    Headers::X_AMZ_CONTENT_SHA256,
    Headers::X_AMZ_DATE,
];

pub struct PutObject<T: AsRef<str>> {
    pub bucket: T,
    pub name: T,
    pub contents: Vec<u8>,
    pub expires: Option<DateTime<Utc>>,
}

impl<T: AsRef<str>> PutObject<T> {
    pub fn new(bucket: T, name: T, contents: Vec<u8>) -> Self {
        PutObject {
            bucket,
            name,
            contents,
            expires: None,
        }
    }

    pub fn expires(mut self, expires: DateTime<Utc>) -> Self {
        self.expires = Some(expires);
        self
    }
}

impl<T: AsRef<str>> S3Request for PutObject<T> {
    type Response = String;

    fn into_request<AR: AsRef<str>>(
        self,
        uri: Uri,
        access_key: AR,
        signing_key: &SigningKey,
        region: Region,
    ) -> Result<Request<HttpBody>, Error> {
        // Payload hash
        let mut hasher = Sha256::new();
        hasher.input(&self.contents);
        let payload_hash = hex::encode(hasher.result().as_slice());

        let content_md5 = base64::encode(&*md5::compute(&self.contents));

        let datetime = Utc::now();
        let date = format!("{}", datetime.format("%Y%m%dT%H%M%SZ"));

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

        let mut request = Request::builder().method(Method::PUT).uri(uri);

        if let Some(expires) = self.expires {
            request = request.header(Headers::EXPIRES, HeaderValue::from_str(&expires.to_gmt())?);
        }

        request = request.header(Headers::CONTENT_MD5, HeaderValue::from_str(&content_md5)?);
        request = request.header(Headers::HOST, HeaderValue::from_str(&host)?);

        request = request.header(
            Headers::X_AMZ_CONTENT_SHA256,
            HeaderValue::from_str(&payload_hash)?,
        );
        request = request.header(Headers::X_AMZ_DATE, HeaderValue::from_str(&date)?);

        let request = sign_request(
            request,
            &access_key.as_ref(),
            &signing_key,
            region.clone(),
            &HEADERS,
        )?;

        println!("{:#?}", request);

        Ok(request.body(HttpBody::from(self.contents))?)
    }

    fn into_response(
        response: Response<HttpBody>,
    ) -> BoxFuture<'static, Result<Self::Response, Error>> {
        Box::pin(async move {
            let etag: String = response
                .headers()
                .get(Headers::ETAG)
                .ok_or(Error::NoEtagInRespoinse)?
                .to_str()?
                .to_owned();

            Ok(etag)
        })
    }
}
