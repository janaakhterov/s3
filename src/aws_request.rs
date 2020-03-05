use crate::{
    Client,
    Error,
    Region,
    SigningKey,
};
use futures_core::future::BoxFuture;
use hyper::{
    Body,
    Request,
};
use url::Url;

pub trait AwsRequest: Sized + Send {
    type Response;

    fn into_request<T: AsRef<str>>(
        self,
        url: Url,
        access_key: T,
        signing_key: &SigningKey,
        region: Region,
    ) -> Result<Request<Body>, Error>;

    fn into_response(
        response: hyper::Response<Body>,
    ) -> BoxFuture<'static, Result<Self::Response, Error>>;

    fn send<'c>(self, client: &'c Client) -> BoxFuture<'c, Result<Self::Response, Error>>
    where
        Self: 'c,
    {
        Box::pin(async move { client.send(self).await })
    }
}
