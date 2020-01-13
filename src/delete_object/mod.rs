use crate::{
    AwsRequest,
    PayloadHash,
    AwsResponse,
    Error,
    Headers,
    Host,
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

const HEADERS: [&'static str; 3] = [
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
        let request = Request::builder()
            .method(Method::DELETE)
            .host(uri.clone(), self.bucket, self.key)?
            .payload_hash(None)?
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
