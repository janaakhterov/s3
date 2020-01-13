use crate::{
    Acl,
    AwsRequest,
    AwsResponse,
    CacheControl,
    Error,
    Gmt,
    GrantType,
    GrantValue,
    Headers,
    Host,
    OptionalGrants,
    OptionalHeader,
    PayloadHash,
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

// PutObject requset Headers, this list *MUST* be in
// sorted order as it is used in the signing process
// of each request.
const HEADERS: [&str; 11] = [
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
    /// Bucket name to which the PUT operation was initiated.
    pub bucket: T,

    /// Object key for which the PUT operation was initiated.
    pub key: T,

    contents: Vec<u8>,
    expires: Option<DateTime<Utc>>,
    grants: Vec<(GrantType, GrantValue, T)>,
    cache: Option<CacheControl<T>>,
    acl: Option<Acl>,
}

impl<T: AsRef<str>> PutObject<T> {
    /// Create a new PutObject request with default parameters
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

    /// The date and time at which the object is no longer cacheable.
    pub fn expires(mut self, expires: DateTime<Utc>) -> Self {
        self.expires = Some(expires);
        self
    }

    /// Can be used to specify caching behavior along the request/reply chain.
    pub fn cache(mut self, cache: CacheControl<T>) -> Self {
        self.cache = Some(cache);
        self
    }

    /// The canned ACL to apply to the object.
    /// Note: Granting explicit permission will overwrite this setting
    pub fn acl(mut self, acl: Acl) -> Self {
        self.acl = Some(acl);
        self
    }

    /// Allows grantee email to read the object data and its metadata.
    pub fn grant_read_email(mut self, email: T) -> Self {
        self.grants
            .push((GrantType::Read, GrantValue::Email, email));
        self
    }

    /// Allows grantee id to read the object data and its metadata.
    pub fn grant_read_id(mut self, id: T) -> Self {
        self.grants.push((GrantType::Read, GrantValue::Id, id));
        self
    }

    /// Allows uri to read the object data and its metadata.
    pub fn grant_read_uri(mut self, uri: T) -> Self {
        self.grants.push((GrantType::Read, GrantValue::Uri, uri));
        self
    }

    /// Allows grantee email to write the ACL for the applicable object.
    pub fn grant_write_acp_email(mut self, email: T) -> Self {
        self.grants
            .push((GrantType::WriteAcp, GrantValue::Email, email));
        self
    }

    /// Allows grantee id to write the ACL for the applicable object.
    pub fn grant_write_acp_id(mut self, id: T) -> Self {
        self.grants.push((GrantType::WriteAcp, GrantValue::Id, id));
        self
    }

    /// Allows grantee uri to write the ACL for the applicable object.
    pub fn grant_write_acp_uri(mut self, uri: T) -> Self {
        self.grants
            .push((GrantType::WriteAcp, GrantValue::Uri, uri));
        self
    }

    /// Allows grantee email to read the object ACL.
    pub fn grant_read_acp_email(mut self, email: T) -> Self {
        self.grants
            .push((GrantType::ReadAcp, GrantValue::Email, email));
        self
    }

    /// Allows grantee id to read the object ACL.
    pub fn grant_read_acp_id(mut self, id: T) -> Self {
        self.grants.push((GrantType::ReadAcp, GrantValue::Id, id));
        self
    }

    /// Allows uri to read the object ACL.
    pub fn grant_read_acp_uri(mut self, uri: T) -> Self {
        self.grants.push((GrantType::ReadAcp, GrantValue::Uri, uri));
        self
    }

    /// Gives the grantee email READ, READ_ACP, and WRITE_ACP permissions on the object.
    pub fn grant_full_email(mut self, email: T) -> Self {
        self.grants
            .push((GrantType::FullControl, GrantValue::Email, email));
        self
    }

    /// Gives the grantee id READ, READ_ACP, and WRITE_ACP permissions on the object.
    pub fn grant_full_id(mut self, id: T) -> Self {
        self.grants
            .push((GrantType::FullControl, GrantValue::Id, id));
        self
    }

    /// Gives the uri READ, READ_ACP, and WRITE_ACP permissions on the object.
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
            .host(uri, self.bucket, self.key)?
            .optional_header(Headers::EXPIRES, &self.expires.map(|since| since.to_gmt()))?
            .optional_header(Headers::CACHE_CONTROL, &cache)?
            .optional_grants(self.acl, self.grants)?
            .header(Headers::CONTENT_MD5, HeaderValue::from_str(&content_md5)?)
            .payload_hash(Some(&self.contents))?
            .sign(&access_key.as_ref(), &signing_key, region.clone(), &HEADERS)?;

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
