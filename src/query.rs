use http::{
    request::Builder,
    uri::{
        PathAndQuery,
        Uri,
    },
    Error,
};
use std::{
    collections::HashMap,
    convert::TryFrom,
};

pub struct QueryParameter;

impl QueryParameter {
    pub const ENCRYPTION: &'static str = "encryption";
    pub const INVENTORY: &'static str = "inventory";
    pub const METRICS: &'static str = "metrics";
    pub const POLICY: &'static str = "policy";
    pub const REPLICATION: &'static str = "replication";
    pub const TAGGING: &'static str = "tagging";
    pub const WEBSITE: &'static str = "website";
    pub const ID: &'static str = "Id";
}

pub trait QueryParam {
    fn query_param(self, params: impl IntoQueryParams) -> Result<Self, Error>
    where
        Self: Sized;
}

impl QueryParam for Builder {
    fn query_param(self, params: impl IntoQueryParams) -> Result<Self, Error>
    where
        Self: Sized,
    {
        let mut parts = self
            .uri_ref()
            .map(|value| (*value).clone())
            .unwrap()
            .into_parts();
        parts.path_and_query = Some(PathAndQuery::try_from(params.into_query_params().as_str())?);
        Ok(self.uri(Uri::from_parts(parts)?))
    }
}

pub trait IntoQueryParams {
    fn into_query_params(self) -> String;
}

impl<T, V> IntoQueryParams for HashMap<T, Option<V>>
where
    T: AsRef<str>,
    V: AsRef<str>,
{
    fn into_query_params(self) -> String {
        format!(
            "?{}",
            self.iter()
                .map(|(key, value)| {
                    if let Some(value) = value {
                        format!("{}={}", key.as_ref(), value.as_ref())
                    } else {
                        key.as_ref().to_owned()
                    }
                })
                .collect::<Vec<String>>()
                .join("&")
        )
    }
}

impl<T, V> IntoQueryParams for (T, V)
where
    T: AsRef<str>,
    V: AsRef<str>,
{
    fn into_query_params(self) -> String {
        format!("?{}={}", self.0.as_ref(), self.1.as_ref())
    }
}

impl IntoQueryParams for &'_ str {
    fn into_query_params(self) -> String {
        format!("?{}", self)
    }
}

impl IntoQueryParams for &String {
    fn into_query_params(self) -> String {
        format!("?{}", self)
    }
}

impl<T, V> IntoQueryParams for &[(T, Option<V>)]
where
    T: AsRef<str>,
    V: AsRef<str>,
{
    fn into_query_params(self) -> String {
        format!(
            "?{}",
            self.iter()
                .map(|(key, value)| {
                    if let Some(value) = value {
                        format!("{}={}", key.as_ref(), value.as_ref())
                    } else {
                        format!("{}", key.as_ref())
                    }
                })
                .collect::<Vec<String>>()
                .join("&")
        )
    }
}
