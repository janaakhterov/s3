use crate::{
    Acl,
    AwsRequest,
    AwsResponse,
    PayloadHash,
    CacheControl,
    Error,
    Gmt,
    GrantType,
    GrantValue,
    Headers,
    Host,
    OptionalGrants,
    OptionalHeader,
    Region,
    SignRequest,
    SigningKey,
};
use chrono::{
    DateTime,
    Utc,
};
use futures_core::future::BoxFuture;
use http::{
    header::HeaderValue,
    method::Method,
    uri::Uri,
};
use hyper::{
    Body as HttpBody,
    Request,
    Response,
};

// This static variable must have all the headers
// in sorted order
const HEADERS: [&'static str; 11] = [
    Headers::CONTENT_MD5,
    Headers::EXPIRES,
    Headers::HOST,
    Headers::X_AMZ_ACL,
    Headers::X_AMZ_CONTENT_SHA256,
    Headers::X_AMZ_DATE,
    Headers::X_AMZ_GRANT_WRITE,
    Headers::X_AMZ_GRANT_READ,
    Headers::X_AMZ_GRANT_WRITE_ACP,
    Headers::X_AMZ_GRANT_READ_ACP,
    Headers::X_AMZ_GRANT_FULL_CONTROL,
];

pub struct PutObject<T: AsRef<str>> {
    pub bucket: T,
    pub key: T,
    pub contents: Vec<u8>,
    pub expires: Option<DateTime<Utc>>,
    pub grants: Vec<(GrantType, GrantValue, T)>,
    pub cache: Option<CacheControl<T>>,
    pub acl: Option<Acl>,
}

impl<T: AsRef<str>> PutObject<T> {
    pub fn new(bucket: T, key: T, contents: Vec<u8>) -> Self {
        PutObject {
            bucket,
            key,
            contents,
            expires: None,
            grants: Vec::new(),
            cache: None,
            acl: None,
        }
    }

    pub fn expires(mut self, expires: DateTime<Utc>) -> Self {
        self.expires = Some(expires);
        self
    }

    pub fn cache(mut self, cache: CacheControl<T>) -> Self {
        self.cache = Some(cache);
        self
    }

    /// Note: Granting explicit permission will overwrite this setting
    pub fn acl(mut self, acl: Acl) -> Self {
        self.acl = Some(acl);
        self
    }

    pub fn grant_read_email(mut self, email: T) -> Self {
        self.grants
            .push((GrantType::Read, GrantValue::Email, email));
        self
    }

    pub fn grant_read_id(mut self, id: T) -> Self {
        self.grants.push((GrantType::Read, GrantValue::Id, id));
        self
    }

    pub fn grant_read_uri(mut self, uri: T) -> Self {
        self.grants.push((GrantType::Read, GrantValue::Uri, uri));
        self
    }

    pub fn grant_write_acp_email(mut self, email: T) -> Self {
        self.grants
            .push((GrantType::WriteAcp, GrantValue::Email, email));
        self
    }

    pub fn grant_write_acp_id(mut self, id: T) -> Self {
        self.grants.push((GrantType::WriteAcp, GrantValue::Id, id));
        self
    }

    pub fn grant_write_acp_uri(mut self, uri: T) -> Self {
        self.grants
            .push((GrantType::WriteAcp, GrantValue::Uri, uri));
        self
    }

    pub fn grant_read_acp_email(mut self, email: T) -> Self {
        self.grants
            .push((GrantType::ReadAcp, GrantValue::Email, email));
        self
    }

    pub fn grant_read_acp_id(mut self, id: T) -> Self {
        self.grants.push((GrantType::ReadAcp, GrantValue::Id, id));
        self
    }

    pub fn grant_read_acp_uri(mut self, uri: T) -> Self {
        self.grants.push((GrantType::ReadAcp, GrantValue::Uri, uri));
        self
    }

    pub fn grant_full_email(mut self, email: T) -> Self {
        self.grants
            .push((GrantType::FullControl, GrantValue::Email, email));
        self
    }

    pub fn grant_full_id(mut self, id: T) -> Self {
        self.grants
            .push((GrantType::FullControl, GrantValue::Id, id));
        self
    }

    pub fn grant_full_uri(mut self, uri: T) -> Self {
        self.grants
            .push((GrantType::FullControl, GrantValue::Uri, uri));
        self
    }
}

impl<T: AsRef<str>> AwsRequest for PutObject<T> {
    type Response = String;

    fn into_request<AR: AsRef<str>>(
        self,
        uri: Uri,
        access_key: AR,
        signing_key: &SigningKey,
        region: Region,
    ) -> Result<Request<HttpBody>, Error> {
        let content_md5 = base64::encode(&*md5::compute(&self.contents));
        let cache = self.cache.map(|cache| {
            let cache: String = cache.into();
            cache
        });

        let request = Request::builder()
            .method(Method::PUT)
            .host(uri.clone(), self.bucket, self.key)?
            .optional_header(Headers::EXPIRES, &self.expires.map(|since| since.to_gmt()))?
            .optional_header(Headers::CACHE_CONTROL, &cache)?
            .optional_grants(self.acl, self.grants)?
            .header(Headers::CONTENT_MD5, HeaderValue::from_str(&content_md5)?)
            .payload_hash(Some(&self.contents))?
            .sign(&access_key.as_ref(), &signing_key, region.clone(), &HEADERS)?;

        println!("{:#?}", request);

        Ok(request.body(HttpBody::from(self.contents))?)
    }

    fn into_response(
        mut response: Response<HttpBody>,
    ) -> BoxFuture<'static, Result<Self::Response, Error>> {
        Box::pin(async move {
            response.error().await?;

            let etag = response.etag()?;

            Ok(etag)
        })
    }
}
