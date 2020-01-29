use crate::{
    error,
    Acl,
    AwsRequest,
    AwsResponse,
    CacheControl,
    Error,
    Gmt,
    Permission,
    Grantee,
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
use http:: method::Method;
use hyper::{
    Body as HttpBody,
    Request,
    Response,
};
use url::Url;

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

pub struct PutObject<'a> {
    /// Bucket name to which the PUT operation was initiated.
    pub bucket: &'a str,

    /// Object key for which the PUT operation was initiated.
    pub key: &'a str,

    contents: Vec<u8>,
    expires: Option<DateTime<Utc>>,
    grants: Vec<(Permission, Grantee, &'a str)>,
    cache: Option<CacheControl<'a>>,
    acl: Option<Acl>,
}

impl<'a> PutObject<'a> {
    /// Create a new PutObject request with default parameters
    pub fn new(bucket: &'a str, key: &'a str, contents: Vec<u8>) -> Self {
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
    pub fn cache(mut self, cache: CacheControl<'a>) -> Self {
        self.cache = Some(cache);
        self
    }

    /// The canned ACL to apply to the object.
    /// Note: Granting explicit permission will overwrite this setting
    pub fn acl(mut self, acl: Acl) -> Self {
        self.acl = Some(acl);
        self
    }
}

impl<'a> AwsRequest for PutObject<'a> {
    type Response = String;

    fn into_request<AR: AsRef<str>>(
        self,
        url: Url,
        access_key: AR,
        signing_key: &SigningKey,
        region: Region,
    ) -> Result<Request<HttpBody>, Error> {
        let cache = self.cache.map(|cache| {
            let cache: String = cache.into();
            cache
        });

        let request = Request::builder()
            .method(Method::PUT)
            .host(url, self.bucket, self.key, None)?
            .optional_header(Headers::EXPIRES, &self.expires.map(|since| since.to_gmt()))?
            .optional_header(Headers::CACHE_CONTROL, &cache)?
            .optional_grants(self.acl, self.grants)?
            .payload_hash(Some(&self.contents))?
            .sign(&access_key.as_ref(), &signing_key, region.clone(), &HEADERS)?;

        Ok(request.body(HttpBody::from(self.contents)).map_err(error::Internal::from)?)
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

macro_rules! grant_method {
    ($ty: ty, $fn: ident, $permission: ident, $grantee: ident) => {
        impl<'a> $ty {
            pub fn $fn(mut self, value: &'a str) -> Self {
                self.grants
                    .push((Permission::$permission, Grantee::$grantee, value));
                self
            }
        }
    };
}

grant_method!(PutObject<'a>, grant_read_id, WriteAcp, Id);
grant_method!(PutObject<'a>, grant_read_acp_id, ReadAcp, Id);
grant_method!(PutObject<'a>, grant_write_acp_id, Read, Id);
grant_method!(PutObject<'a>, grant_full_id, FullControl, Id);

grant_method!(PutObject<'a>, grant_read_email, WriteAcp, Email);
grant_method!(PutObject<'a>, grant_read_acp_email, ReadAcp, Email);
grant_method!(PutObject<'a>, grant_write_acp_email, Read, Email);
grant_method!(PutObject<'a>, grant_full_email, FullControl, Email);

grant_method!(PutObject<'a>, grant_read_uri, WriteAcp, Uri);
grant_method!(PutObject<'a>, grant_read_acp_uri, ReadAcp, Uri);
grant_method!(PutObject<'a>, grant_write_acp_uri, Read, Uri);
grant_method!(PutObject<'a>, grant_full_uri, FullControl, Uri);
