use http::{
    request::Builder,
    uri::{
        PathAndQuery,
        Uri,
    },
    Error,
};
use std::convert::TryFrom;

pub struct QueryParameter;

impl QueryParameter {
    pub const ENCRYPTION: &'static str = "encryption";
}

pub trait QueryParam {
    fn query_param(self, key: &str, value: &str) -> Result<Self, Error>
    where
        Self: Sized;
}

impl QueryParam for Builder {
    fn query_param(self, key: &str, value: &str) -> Result<Self, Error>
    where
        Self: Sized,
    {
        let mut parts = self
            .uri_ref()
            .map(|value| (*value).clone())
            .unwrap()
            .into_parts();
        let path_and_query = parts.path_and_query.unwrap();
        let path = if let Some(query) = path_and_query.query() {
            format!("{}{}&{}={}", path_and_query.path(), query, key, value)
        } else {
            format!("{}?{}={}", path_and_query.path(), key, value)
        };

        parts.path_and_query = Some(PathAndQuery::try_from(path.as_str())?);
        Ok(self.uri(Uri::from_parts(parts)?))
    }
}
