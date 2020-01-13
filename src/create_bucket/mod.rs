use crate::{
    Acl,
    AwsRequest,
    AwsResponse,
    Error,
    GrantType,
    GrantValue,
    Headers,
    Host,
    OptionalGrants,
    PayloadHash,
    Region,
    SignRequest,
    SigningKey,
};
use futures_core::future::BoxFuture;
use http::{
    method::Method,
    uri::Uri,
};
use hyper::{
    Body as HttpBody,
    Request,
    Response,
};
use quick_xml::se::to_string;
use serde::Serialize;

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

#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct CreateBucketConfiguration {
    // Currently quick-xml does not support unit variants
    // like the `Region` enum, so a work around is to use
    // `String`
    location_constraint: Option<String>,
}

pub struct CreateBucket<T: AsRef<str>> {
    /// The name of the bucket to create.
    pub bucket: T,

    grants: Vec<(GrantType, GrantValue, T)>,
    acl: Option<Acl>,
    location: Option<Region>,
}

impl<T: AsRef<str>> CreateBucket<T> {
    /// Create a new CreateBucket request with the given bucket name.
    pub fn new(bucket: T) -> Self {
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

impl<T: AsRef<str>> AwsRequest for CreateBucket<T> {
    type Response = ();

    fn into_request<AR: AsRef<str>>(
        self,
        uri: Uri,
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

        let payload = to_string(&config)?;

        let request = Request::builder()
            .method(Method::PUT)
            .host(uri, self.bucket, "")?
            .optional_grants(self.acl, self.grants)?
            .payload_hash(Some(&payload.as_bytes()))?
            .sign(&access_key.as_ref(), &signing_key, region.clone(), &HEADERS)?;

        Ok(request.body(HttpBody::from(payload))?)
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
