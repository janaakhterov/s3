use crate::{Error, Region, SigningKey};
use futures_core::future::BoxFuture;
use reqwest::{Request, Url};
use std::fmt::Display;

pub trait AmzRequest {
    type Response;

    fn into_request<T: AsRef<str> + Display>(
        self,
        url: Url,
        access_key: T,
        signing_key: &SigningKey,
        region: Region,
    ) -> Result<Request, Error>;

    fn into_response(
        response: reqwest::Response,
    ) -> BoxFuture<'static, Result<Self::Response, Error>>;
}
