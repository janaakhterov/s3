use crate::{
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
use http::{
    header::HeaderValue,
    method::Method,
};
use hyper::{
    Body as HttpBody,
    Request,
    Response,
};
use serde::Serialize;
use url::Url;

// PutBucketEncryption requset Headers, this list *MUST* be in
// sorted order as it is used in the signing process
// of each request.
const HEADERS: [&str; 4] = [
    Headers::CONTENT_MD5,
    Headers::HOST,
    Headers::X_AMZ_CONTENT_SHA256,
    Headers::X_AMZ_DATE,
];

#[derive(Debug, Serialize)]
struct ServerSideEncryptionConfiguration {
    #[serde(rename = "Rule")]
    rule: Rule,
}

#[derive(Default, Debug, Serialize)]
struct Rule {
    #[serde(rename = "SSEAlgorithm")]
    sse: Option<String>,
    #[serde(rename = "KMSMasterKeyID")]
    kms_key: Option<String>,
}

pub enum AwsEncryption<T: AsRef<str>> {
    Sse,
    Kms(T),
}

impl<T: AsRef<str>> Into<&'static str> for AwsEncryption<T> {
    fn into(self) -> &'static str {
        match self {
            AwsEncryption::Sse => "AES256",
            AwsEncryption::Kms(_) => "aws:kms",
        }
    }
}

pub struct PutBucketEncryption<T: AsRef<str>> {
    /// Bucket name to which the PUT operation was initiated.
    pub bucket: T,

    encryption: Option<AwsEncryption<T>>,
}

impl<T: AsRef<str>> PutBucketEncryption<T> {
    /// Create a new PutBucketEncryption request with default parameters
    pub fn new(bucket: T) -> Self {
        PutBucketEncryption {
            bucket,
            encryption: None,
        }
    }

    pub fn encrypt_with_sse(mut self) -> Self {
        self.encryption = Some(AwsEncryption::Sse);
        self
    }

    pub fn encrypt_with_kms(mut self, key: T) -> Self {
        self.encryption = Some(AwsEncryption::Kms(key));
        self
    }
}

impl<T: AsRef<str>> AwsRequest for PutBucketEncryption<T> {
    type Response = ();

    fn into_request<AR: AsRef<str>>(
        self,
        url: Url,
        access_key: AR,
        signing_key: &SigningKey,
        region: Region,
    ) -> Result<Request<HttpBody>, Error> {
        let mut config = ServerSideEncryptionConfiguration {
            rule: Rule::default(),
        };

        if let Some(encryption) = self.encryption {
            if let AwsEncryption::Kms(ref key) = encryption {
                config.rule.kms_key = Some(key.as_ref().to_string());
            }
            let s: &'static str = encryption.into();
            config.rule.sse = Some(s.to_owned());
        }

        let payload = quick_xml::se::to_string(&config)?;

        println!("{}", payload);

        let content_md5 = base64::encode(&*md5::compute(&payload));

        let request = Request::builder()
            .method(Method::PUT)
            .host(url, self.bucket, "", Some(region))?
            .query_param(QueryParameter::ENCRYPTION, "")?
            .header(Headers::CONTENT_MD5, HeaderValue::from_str(&content_md5)?)
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
