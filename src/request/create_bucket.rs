use crate::{
    error,
    Acl,
    AwsRequest,
    AwsResponse,
    Error,
    Permission,
    Grantee,
    Headers,
    Host,
    OptionalGrants,
    PayloadHash,
    Region,
    SignRequest,
    SigningKey,
};
use futures_core::future::BoxFuture;
use http::method::Method;
use hyper::{
    Body as HttpBody,
    Request,
    Response,
};
use quick_xml::se::to_string;
use url::Url;
use crate::types::bucket::CreateBucketConfiguration;

// CreateBucket request Headers, this list *MUST* be in
// sorted order as it is used in the signing process
// of each request.
const HEADERS: [&str; 9] = [
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

pub struct CreateBucket<'a> {
    /// The name of the bucket to create.
    pub bucket: &'a str,

    grants: Vec<(Permission, Grantee, &'a str)>,
    acl: Option<Acl>,
    location: Option<Region>,
}

impl<'a> CreateBucket<'a> {
    /// Create a new CreateBucket request with the given bucket name.
    pub fn new(bucket: &'a str) -> Self {
        CreateBucket {
            bucket,
            grants: Vec::new(),
            acl: None,
            location: None,
        }
    }

    /// Specifies the Region where the bucket will be created. If you don't specify a Region, the bucket is created in the US East (N. Virginia) Region (us-east-1).
    pub fn location(mut self, location: Region) -> Self {
        self.location = Some(location);
        self
    }

    /// The canned ACL to apply to the object.
    /// **Note:** Granting explicit permission will overwrite this setting
    pub fn acl(mut self, acl: Acl) -> Self {
        self.acl = Some(acl);
        self
    }
}

impl<'a> AwsRequest for CreateBucket<'a> {
    type Response = ();

    fn into_request<AR: AsRef<str>>(
        self,
        uri: Url,
        access_key: AR,
        signing_key: &SigningKey,
        region: Region,
    ) -> Result<Request<HttpBody>, Error> {
        let mut config = CreateBucketConfiguration {
            location_constraint: None,
        };

        if let Some(location) = self.location {
            // Region::UsEast1 is the default value
            // and *cannot* be used as a value for location
            if location != Region::UsEast1 {
                let location: String = location.into();
                config.location_constraint = Some(location);
            }
        }

        let payload = to_string(&config)
            .map_err(error::Internal::from)?;

        let request = Request::builder()
            .method(Method::PUT)
            .host(uri, self.bucket, "", None)?
            .optional_grants(self.acl, self.grants)?
            .payload_hash(Some(&payload.as_bytes()))?
            .sign(&access_key.as_ref(), &signing_key, region.clone(), &HEADERS)?;

        Ok(request.body(HttpBody::from(payload)).map_err(error::Internal::from)?)
    }

    fn into_response(
        mut response: Response<HttpBody>,
    ) -> BoxFuture<'static, Result<Self::Response, Error>> {
        Box::pin(async move {
            response.error().await?;

            Ok(())
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

grant_method!(CreateBucket<'a>, grant_read_id, WriteAcp, Id);
grant_method!(CreateBucket<'a>, grant_read_acp_id, ReadAcp, Id);
grant_method!(CreateBucket<'a>, grant_write_acp_id, Read, Id);
grant_method!(CreateBucket<'a>, grant_full_id, FullControl, Id);

grant_method!(CreateBucket<'a>, grant_read_email, WriteAcp, Email);
grant_method!(CreateBucket<'a>, grant_read_acp_email, ReadAcp, Email);
grant_method!(CreateBucket<'a>, grant_write_acp_email, Read, Email);
grant_method!(CreateBucket<'a>, grant_full_email, FullControl, Email);

grant_method!(CreateBucket<'a>, grant_read_uri, WriteAcp, Uri);
grant_method!(CreateBucket<'a>, grant_read_acp_uri, ReadAcp, Uri);
grant_method!(CreateBucket<'a>, grant_write_acp_uri, Read, Uri);
grant_method!(CreateBucket<'a>, grant_full_uri, FullControl, Uri);
