use crate::{
    Host,
    PayloadHash,
    AwsRequest,
    AwsResponse,
    Error,
    Headers,
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

pub struct DeleteBucket<T: AsRef<str>> {
    bucket: T,
}

impl<T: AsRef<str>> DeleteBucket<T> {
    pub fn new(bucket: T) -> Self {
        DeleteBucket { bucket }
    }
}


impl<T: AsRef<str>> AwsRequest for DeleteBucket<T> {
    type Response = ();

    fn into_request<AR: AsRef<str>>(
        self,
        uri: Uri,
        access_key: AR,
        signing_key: &SigningKey,
        region: Region,
    ) -> Result<Request<HttpBody>, Error> {
        let request = Request::builder()
            .method(Method::PUT)
            .host(uri.clone(), self.bucket, "")?
            .payload_hash(None)?
            .sign(&access_key.as_ref(), &signing_key, region.clone(), &HEADERS)?;

        println!("{:#?}", request);

        Ok(request.body(HttpBody::empty())?)
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
