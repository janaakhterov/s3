use crate::{
    Error,
    Headers,
    Region,
    SigningKey,
};
use chrono::NaiveDateTime;
use http::request::Builder;
use hyper::header::HeaderValue;
use sha2::{
    Digest,
    Sha256,
};

pub trait SignRequest {
    fn sign<T: AsRef<str>>(
        self,
        access_key: T,
        signing_key: &SigningKey,
        region: Region,
        headers: &'static [&'static str],
    ) -> Result<Self, Error>
    where
        Self: Sized;
}

impl SignRequest for Builder {
    fn sign<T: AsRef<str>>(
        self,
        access_key: T,
        signing_key: &SigningKey,
        region: Region,
        headers: &'static [&'static str],
    ) -> Result<Self, Error>
    where
        Self: Sized,
    {
        let mut canonical: Vec<u8> = Vec::new();
        let mut signed: Vec<&str> = Vec::new();

        // Request Method
        canonical.extend_from_slice(
            &self
                .method_ref()
                .ok_or(Error::MethodNotSet)?
                .as_str()
                .as_bytes(),
        );
        canonical.push(b'\n');

        // Resource
        canonical.extend_from_slice(&self.uri_ref().ok_or(Error::UriNotSet)?.path().as_bytes());
        canonical.push(b'\n');

        // TODO: QueryParameters
        canonical.push(b'\n');

        for header in headers {
            if let Some(value) = self.headers_ref().ok_or(Error::HeadersNotSet)?.get(*header) {
                canonical.extend_from_slice(&header.as_bytes());
                canonical.push(b':');
                canonical.extend_from_slice(&value.as_bytes());
                canonical.push(b'\n');

                signed.push(header)
            }
        }

        // End of Headers
        canonical.push(b'\n');

        let signed = signed.join(";");

        // Signed Headers
        canonical.extend_from_slice(&signed.as_bytes());

        canonical.push(b'\n');

        // X_AMZ_CONTENT_SHA256 should ALWAYS be set
        if let Some(header) = self
            .headers_ref()
            .ok_or(Error::HeadersNotSet)?
            .get(Headers::X_AMZ_CONTENT_SHA256)
        {
            canonical.extend_from_slice(&header.as_bytes());
        }

        let mut hasher = Sha256::new();
        hasher.input(canonical);

        let hash = hex::encode(hasher.result().as_slice());

        let region: String = region.into();

        let date = self
            .headers_ref()
            .ok_or(Error::HeadersNotSet)?
            .get(Headers::X_AMZ_DATE)
            .ok_or(Error::DateHeaderUnsetWhenSigning)?
            .to_str()?;

        let date = NaiveDateTime::parse_from_str(date, "%Y%m%dT%H%M%SZ")?;

        let scope = format!(
            "{date}/{region}/s3/aws4_request",
            date = date.format("%Y%m%d"),
            region = region
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
        access_key = access_key.as_ref(),
        scope = scope,
        signed_headers = signed,
        signature = sig
    );

        Ok(self.header(Headers::AUTHORIZATION, HeaderValue::from_str(&auth)?))
    }
}
