use reqwest::header::InvalidHeaderValue;
use serde::Deserialize;
use thiserror::Error;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(rename = "Error")]
pub struct ResponseError {
    code: String,
    message: String,
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("Error sending request")]
    RequestError(#[from] reqwest::Error),

    #[error("Received an error while executing a request: {0:?}")]
    ResponseError(ResponseError),

    #[error("Invalid header value provided: {0:?}")]
    InvalidHeaderValue(#[from] InvalidHeaderValue),

    #[error("Failed to deserialize response error")]
    DeserializeError(#[from] quick_xml::DeError),

    #[error("Failed to build client because not all fields were provided")]
    ClientBuildError,

    #[error("Failed to parse url")]
    ParseError(#[from] url::ParseError),
}
