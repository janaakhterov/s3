use crate::{
    Acl,
    PayloadHash,
    Host,
    AwsRequest,
    AwsResponse,
    Error,
    GrantType,
    GrantValue,
    Headers,
    OptionalGrants,
    Region,
    SignRequest,
    SigningKey,
};
use quick_xml::se::to_string;
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
use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct CreateBucketConfiguration {
    // Currently quick-xml does not support unit variants
    // like the `Region` enum, so a work around is to use
    // `String`
    location_constraint: Option<String>
}

const HEADERS: [&'static str; 9] = [
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

pub struct CreateBucket<T: AsRef<str>> {
    bucket: T,
    grants: Vec<(GrantType, GrantValue, T)>,
    acl: Option<Acl>,
    location: Option<Region>,
}

impl<T: AsRef<str>> CreateBucket<T> {
    pub fn new(bucket: T) -> Self {
        CreateBucket {
            bucket,
            grants: Vec::new(),
            acl: None,
            location: None,
        }
    }

    pub fn location(mut self, location: Region) -> Self {
        self.location = Some(location);
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


impl<T: AsRef<str>> AwsRequest for CreateBucket<T> {
    type Response = ();

    fn into_request<AR: AsRef<str>>(
        self,
        uri: Uri,
        access_key: AR,
        signing_key: &SigningKey,
        region: Region,
    ) -> Result<Request<HttpBody>, Error> {
        let mut config = CreateBucketConfiguration { location_constraint: None, };

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
            .host(uri.clone(), self.bucket, "")?
            .optional_grants(self.acl, self.grants)?
            .payload_hash(Some(&payload.as_bytes()))?
            .sign(&access_key.as_ref(), &signing_key, region.clone(), &HEADERS)?;

        println!("{:#?}", payload);
        println!("{:#?}", request);

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
