use hyper::header::InvalidHeaderValue;
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
    RequestError(#[from] hyper::Error),

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
    DateHeaderToStrError(#[from] hyper::header::ToStrError),

    #[error("Failed to parse chrono datetime: {0:?}")]
    ChronoParseError(#[from] chrono::ParseError),

    #[error("Failed to a number in header")]
    ParseIntError(#[from] std::num::ParseIntError),

    #[error("Failed to parse a boolean in header")]
    ParseBoolError(#[from] std::str::ParseBoolError),

    #[error("Invalid URI")]
    InvalidUri(#[from] http::uri::InvalidUri),

    #[error("Http Error")]
    HttpError(#[from] http::Error),

    #[error("Storage class header provided, but failed to parse it")]
    ParseStorageClassError,

    #[error("Method was not set on request during signing, but is required")]
    MethodNotSet,

    #[error("Uri was not set on request during signing, but is required")]
    UriNotSet,

    #[error("Headers was not set on request during signing, but is required")]
    HeadersNotSet,

    #[error("Aws Response did not have an etag header present")]
    NoEtagInResponse,

    #[error("Invalid uri parts")]
    InvalidUriParts(#[from] http::uri::InvalidUriParts),

    #[error("last-modified header is not present on the response to a get object request")]
    LastModifiedNotPresentOnGetResponse,

    #[cfg(feature = "credential_file")]
    #[error("Failed to parse aws credentials file")]
    AwsCredentialsParseError,

    #[error("Could not find credentials from environment variables or credentials file")]
    CouldNotFindCredentials,

    #[error("Found credentials file, but could not find 'aws_access_key_id' *and* 'aws_secret_access_key'")]
    AwsCredentialsNotFound,
}
