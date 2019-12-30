use crate::{
    Error,
    Region,
    SigningKey,
};
use futures_core::future::BoxFuture;
use hyper::{
    Body,
    Request,
    Uri,
};

pub trait AwsRequest {
    type Response;

    fn into_request<T: AsRef<str>>(
        self,
        url: Uri,
        access_key: T,
        signing_key: &SigningKey,
        region: Region,
    ) -> Result<Request<Body>, Error>;

    fn into_response(
        response: hyper::Response<Body>,
    ) -> BoxFuture<'static, Result<Self::Response, Error>>;
}
