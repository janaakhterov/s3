use http::{
    header::{
        HeaderName,
        HeaderValue,
    },
    request::Builder,
    Error,
};
use std::convert::TryFrom;

pub trait OptionHeader {
    fn option_header<K, V>(self, key: K, value: &Option<V>) -> Result<Self, Error>
    where
        HeaderName: TryFrom<K>,
        <HeaderName as TryFrom<K>>::Error: Into<Error>,
        V: AsRef<str>,
        Self: Sized;
}

impl OptionHeader for Builder {
    fn option_header<K, V>(self, key: K, value: &Option<V>) -> Result<Self, Error>
    where
        HeaderName: TryFrom<K>,
        <HeaderName as TryFrom<K>>::Error: Into<Error>,
        V: AsRef<str>,
        Self: Sized,
    {
        if let Some(value) = value {
            Ok(self.header(key, HeaderValue::from_str(&value.as_ref())?))
        } else {
            Ok(self)
        }
    }
}
