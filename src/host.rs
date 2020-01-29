use crate::{
    error,
    Headers,
    Region,
};
use http::{
    header::HeaderValue,
    request::Builder,
    uri::Uri,
};
use std::convert::TryFrom;
use url::Url;

pub trait Host {
    fn host<B: AsRef<str>, K: AsRef<str>>(
        self,
        url: Url,
        bucket: B,
        key: K,
        region: Option<Region>,
    ) -> Result<Self, error::Error>
    where
        Self: Sized;
}

impl Host for Builder {
    fn host<B: AsRef<str>, K: AsRef<str>>(
        self,
        url: Url,
        bucket: B,
        key: K,
        region: Option<Region>,
    ) -> Result<Self, error::Error> {
        let domain = if bucket.as_ref() != "" {
            format!("{}.{}", bucket.as_ref(), url.domain().unwrap())
        } else {
            url.domain().unwrap().to_owned()
        };

        let uri = format!(
            "{}://{}:{}/{}",
            url.scheme(),
            domain,
            url.port().map(|v| v.to_string()).unwrap_or("".to_owned()),
            key.as_ref()
        );

        let domain = if let Some(region) = region {
            let region: String = region.into();

            format!("{}.{}.{}", bucket.as_ref(), region, url.domain().unwrap())
        } else {
            domain
        };

        let uri = Uri::try_from(&uri).map_err(error::Internal::from)?;

        Ok(self.uri(uri).header(
            Headers::HOST,
            HeaderValue::from_str(&domain).map_err(error::Internal::from)?,
        ))
    }
}
