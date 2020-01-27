use crate::{
    AwsRequest,
    AwsResponse,
    Error,
    Headers,
    Host,
    PayloadHash,
    QueryParam,
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

const HEADERS: [&str; 3] = [
    Headers::HOST,
    Headers::X_AMZ_CONTENT_SHA256,
    Headers::X_AMZ_DATE,
];

pub(crate) struct SubResource<T: AsRef<str>, V: AsRef<str>> {
    pub(crate) bucket: T,
    pub(crate) method: Method,
    pub(crate) key: Option<String>,
    pub(crate) params: Vec<(&'static str, Option<V>)>,
}

impl<T: AsRef<str>, V: AsRef<str>> AwsRequest for SubResource<T, V> {
    type Response = ();

    fn into_request<AR: AsRef<str>>(
        self,
        url: Url,
        access_key: AR,
        signing_key: &SigningKey,
        region: Region,
    ) -> Result<Request<HttpBody>, Error> {
        let request = Request::builder()
            .method(self.method)
            .host(url, self.bucket, self.key.unwrap_or(String::new()), None)?
            .query_param(&self.params[..])?
            .payload_hash(None)?
            .sign(&access_key.as_ref(), &signing_key, region.clone(), &HEADERS)?;

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
