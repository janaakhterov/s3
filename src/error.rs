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

    #[error("x-amz-date was not set when signing a request")]
    DateHeaderUnsetWhenSigning,

    #[error("Host was not set in the url")]
    HostStrUnset,

    #[error("x-amz-date was not encoded correctly")]
    DateHeaderToStrError(#[from] reqwest::header::ToStrError),

    #[error("Failed to parse url")]
    UrlParseError(#[from] url::ParseError),

    #[error("Failed to parse chrono datetime")]
    ChronoParseError(#[from] chrono::ParseError),

    #[error("Failed to a number in header")]
    ParseIntError(#[from] std::num::ParseIntError),

    #[error("Storage class header provided, but failed to parse it")]
    ParseStorageClassError,

    #[error("Aws Response did not have an etag header present")]
    NoEtagInRespoinse,

    #[error("last-modified header is not present on the response to a get object request")]
    LastModifiedNotPresentOnGetResponse,
}
