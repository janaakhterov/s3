use crate::{
    AwsRequest,
    AwsResponse,
    Error,
    Gmt,
    Headers,
    Host,
    OptionHeader,
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
use sha2::{
    Digest,
    Sha256,
};

pub const HEADERS: [&'static str; 5] = [
    Headers::CONTENT_MD5,
    Headers::EXPIRES,
    Headers::HOST,
    Headers::X_AMZ_CONTENT_SHA256,
    Headers::X_AMZ_DATE,
];

pub struct PutObject<T: AsRef<str>> {
    pub bucket: T,
    pub name: T,
    pub contents: Vec<u8>,
    pub expires: Option<DateTime<Utc>>,
}

impl<T: AsRef<str>> PutObject<T> {
    pub fn new(bucket: T, name: T, contents: Vec<u8>) -> Self {
        PutObject {
            bucket,
            name,
            contents,
            expires: None,
        }
    }

    pub fn expires(mut self, expires: DateTime<Utc>) -> Self {
        self.expires = Some(expires);
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
        // Payload hash
        let mut hasher = Sha256::new();
        hasher.input(&self.contents);
        let payload_hash = hex::encode(hasher.result().as_slice());

        let content_md5 = base64::encode(&*md5::compute(&self.contents));

        let datetime = Utc::now();
        let date = format!("{}", datetime.format("%Y%m%dT%H%M%SZ"));

        let request = Request::builder()
            .method(Method::PUT)
            .host(uri.clone(), self.bucket, self.name)?
            .option_header(Headers::EXPIRES, &self.expires.map(|since| since.to_gmt()))?
            .header(Headers::CONTENT_MD5, HeaderValue::from_str(&content_md5)?)
            .header(
                Headers::X_AMZ_CONTENT_SHA256,
                HeaderValue::from_str(&payload_hash)?,
            )
            .header(Headers::X_AMZ_DATE, HeaderValue::from_str(&date)?)
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
