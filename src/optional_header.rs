use crate::error;
use http::{
    header::{
        HeaderName,
        HeaderValue,
    },
    request::Builder,
};
use std::convert::TryFrom;

pub trait OptionalHeader {
    fn optional_header<K, V>(self, key: K, value: &Option<V>) -> Result<Self, error::Error>
    where
        HeaderName: TryFrom<K>,
        <HeaderName as TryFrom<K>>::Error: Into<http::Error>,
        V: AsRef<str>,
        Self: Sized;
}

impl OptionalHeader for Builder {
    fn optional_header<K, V>(self, key: K, value: &Option<V>) -> Result<Self, error::Error>
    where
        HeaderName: TryFrom<K>,
        <HeaderName as TryFrom<K>>::Error: Into<http::Error>,
        V: AsRef<str>,
        Self: Sized,
    {
        if let Some(value) = value {
            Ok(self.header(
                key,
                HeaderValue::from_str(&value.as_ref()).map_err(error::Internal::from)?,
            ))
        } else {
            Ok(self)
        }
    }
}
