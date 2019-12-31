use crate::{
    AwsRequest,
    AwsResponse,
    Error,
    Headers,
    Host,
    Region,
    SignRequest,
    SigningKey,
};
use chrono::Utc;
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

pub const HEADERS: [&'static str; 3] = [
    Headers::HOST,
    Headers::X_AMZ_CONTENT_SHA256,
    Headers::X_AMZ_DATE,
];

pub struct DeleteObject<T: AsRef<str>> {
    pub bucket: T,
    pub key: T,
}

impl<T: AsRef<str>> DeleteObject<T> {
    pub fn new(bucket: T, key: T) -> Self {
        DeleteObject { bucket, key }
    }
}

impl<T: AsRef<str>> AwsRequest for DeleteObject<T> {
    type Response = bool;

    fn into_request<AR: AsRef<str>>(
        self,
        uri: Uri,
        access_key: AR,
        signing_key: &SigningKey,
        region: Region,
    ) -> Result<Request<HttpBody>, Error> {
        let payload_hash = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";

        let date = format!("{}", Utc::now().format("%Y%m%dT%H%M%SZ"));

        let request = Request::builder()
            .method(Method::DELETE)
            .host(uri.clone(), self.bucket, self.key)?
            .header(
                Headers::X_AMZ_CONTENT_SHA256,
                HeaderValue::from_str(&payload_hash)?,
            )
            .header(Headers::X_AMZ_DATE, HeaderValue::from_str(&date)?)
            .sign(&access_key.as_ref(), &signing_key, region.clone(), &HEADERS)?;

        println!("{:#?}", request);

        Ok(request.body(HttpBody::from(HttpBody::empty()))?)
    }

    fn into_response(
        mut response: Response<HttpBody>,
    ) -> BoxFuture<'static, Result<Self::Response, Error>> {
        Box::pin(async move {
            response.error().await?;

            let deleted = response.delete_marker()?.unwrap_or(false);

            Ok(deleted)
        })
    }
}
