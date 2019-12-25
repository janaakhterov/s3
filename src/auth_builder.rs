use crate::{buf_mut::BufMut, region::Region, signing_key::SigningKey};
use chrono::{DateTime, Utc};
use reqwest::header::{HeaderMap, HeaderValue, InvalidHeaderValue};
use sha2::{Digest, Sha256};
use std::error::Error;

pub struct AuthBuilder<'a> {
    region: Region,
    timestamp: DateTime<Utc>,
    canonical: Vec<u8>,
    headers: &'a mut HeaderMap,
    signed: Vec<&'static str>,
}

impl<'a> AuthBuilder<'a> {
    pub fn new(
        headers: &'a mut HeaderMap,
        method: &'static str,
        region: Region,
        timestamp: DateTime<Utc>,
    ) -> Self {
        let mut sig = Self {
            headers,
            region,
            timestamp,
            canonical: Vec::new(),
            signed: Vec::new(),
        };

        sig.set_method(method);

        sig
    }

    pub fn add_header<T: AsRef<str>>(
        &mut self,
        header: &'static str,
        value: T,
    ) -> Result<(), InvalidHeaderValue> {
        self.canonical.add_header(header, &value);
        self.signed.push(header);
        self.headers
            .insert(header, HeaderValue::from_str(value.as_ref())?);

        Ok(())
    }

    fn set_method(&mut self, method: &'static str) {
        self.canonical.extend_from_slice(method.as_bytes());
        self.canonical.push(b'\n');
    }

    pub fn set_resource<T: AsRef<str>>(&mut self, resource: Option<T>) {
        if let Some(resource) = resource {
            self.canonical.push(b'/');
            self.canonical
                .extend_from_slice(resource.as_ref().as_bytes());
        }

        self.canonical.push(b'\n');
    }

    pub fn set_query_params(&mut self) {
        self.canonical.push(b'\n');
    }

    pub fn set_payload(&mut self, payload: &str) {
        self.canonical.extend_from_slice(payload.as_bytes());
    }

    pub fn set_signed_headers(&mut self) {
        self.canonical.push(b'\n');
        self.canonical
            .extend_from_slice(&self.signed.join(";").as_bytes());
        self.canonical.push(b'\n');
    }

    pub fn build(
        self,
        access_key: &str,
        signing_key: &SigningKey,
    ) -> Result<(), InvalidHeaderValue> {
        println!("{}", String::from_utf8_lossy(&self.canonical));

        let mut hasher = Sha256::new();
        hasher.input(self.canonical);
        let canonical = hex::encode(hasher.result().as_slice());

        let region: String = self.region.into();

        let scope = format!(
            "{date}/{region}/s3/aws4_request",
            date = self.timestamp.format("%Y%m%d"),
            region = region
        );

        let string_to_sign = format!(
            "AWS4-HMAC-SHA256\n{timestamp}\n{scope}\n{hash}",
            timestamp = self.timestamp.format("%Y%m%dT%H%M%SZ"),
            scope = scope,
            hash = canonical
        );

        println!("string_to_sign:");
        println!("{}", string_to_sign);
        println!("-------------------------------------------");

        let sig = signing_key.sign(string_to_sign);

        println!("signature:");
        println!("{}", sig);
        println!("-------------------------------------------");

        let auth = format!(
            "AMS4-HMCA-SHA256 Credential={access_key}/{scope},SignedHeaders={signed_headers},Signature={signature}",
            access_key = access_key,
            scope = scope,
            signed_headers = self.signed.join(";"),
            signature = sig
        );

        println!("auth header:");
        println!("{}", auth);
        println!("-------------------------------------------");

        self.headers
            .insert("Authorization", HeaderValue::from_str(&auth)?);

        Ok(())
    }
}
