use crate::{
    error,
    AwsRequest,
    AwsResponse,
    Error,
    Headers,
    Host,
    PayloadHash,
    QueryParam,
    QueryParameter,
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
use url::Url;
use crate::types::BucketEncryption;
use crate::types::Rule;

// PutBucketEncryption requset Headers, this list *MUST* be in
// sorted order as it is used in the signing process
// of each request.
const HEADERS: [&str; 4] = [
    Headers::CONTENT_MD5,
    Headers::HOST,
    Headers::X_AMZ_CONTENT_SHA256,
    Headers::X_AMZ_DATE,
];

pub enum AwsEncryption<'a> {
    Sse,
    Kms(&'a str),
}

impl<'a> Into<&'static str> for AwsEncryption<'a> {
    fn into(self) -> &'static str {
        match self {
            AwsEncryption::Sse => "AES256",
            AwsEncryption::Kms(_) => "aws:kms",
        }
    }
}

pub struct PutBucketEncryption<'a> {
    /// Bucket name to which the PUT operation was initiated.
    pub bucket: &'a str,

    encryption: Option<AwsEncryption<'a>>,
}

impl<'a> PutBucketEncryption<'a> {
    /// Create a new PutBucketEncryption request with default parameters
    pub fn new(bucket: &'a str) -> Self {
        PutBucketEncryption {
            bucket,
            encryption: None,
        }
    }

    pub fn encrypt_with_sse(mut self) -> Self {
        self.encryption = Some(AwsEncryption::Sse);
        self
    }

    pub fn encrypt_with_kms(mut self, key: &'a str) -> Self {
        self.encryption = Some(AwsEncryption::Kms(key));
        self
    }
}

impl<'a> AwsRequest for PutBucketEncryption<'a> {
    type Response = ();

    fn into_request<AR: AsRef<str>>(
        self,
        url: Url,
        access_key: AR,
        signing_key: &SigningKey,
        region: Region,
    ) -> Result<Request<HttpBody>, Error> {
        let mut config = BucketEncryption {
            rule: Rule::default(),
        };

        if let Some(encryption) = self.encryption {
            if let AwsEncryption::Kms(ref key) = encryption {
                config.rule.kms_key = Some(key.to_string());
            }
            let s: &'static str = encryption.into();
            config.rule.sse = Some(s.to_owned());
        }

        let payload = quick_xml::se::to_string(&config).map_err(error::Internal::from)?;

        let request = Request::builder()
            .method(Method::PUT)
            .host(url, self.bucket, "", Some(region))?
            .query_param(QueryParameter::ENCRYPTION)?
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
