use crate::{
    Error,
    Headers,
};
use http::{
    header::HeaderValue,
    request::Builder,
    uri::{
        PathAndQuery,
        Uri,
    },
};
use std::convert::TryFrom;

pub trait Host {
    fn host<B: AsRef<str>, K: AsRef<str>>(self, uri: Uri, bucket: B, key: K) -> Result<Self, Error>
    where
        Self: Sized;
}

impl Host for Builder {
    fn host<B: AsRef<str>, K: AsRef<str>>(
        mut self,
        uri: Uri,
        bucket: B,
        key: K,
    ) -> Result<Self, Error> {
        let resource =
            PathAndQuery::try_from(format!("/{}/{}", bucket.as_ref(), key.as_ref()).as_str())?;

        let mut parts = uri.clone().into_parts();
        parts.path_and_query = Some(resource);
        let uri = Uri::from_parts(parts)?;

        let host = uri.host().ok_or(Error::HostStrUnset)?.to_owned();

        self = self.uri(uri);
        self = self.header(Headers::HOST, HeaderValue::from_str(&host)?);

        Ok(self)
    }
}
