use hyper::header::InvalidHeaderValue;
use serde::Deserialize;
use thiserror::Error;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(rename = "Error")]
pub struct AwsResponseError {
    pub code: String,
    pub message: String,
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, Error)]
#[error("S3 responded with an status code {status}")]
pub struct ResponseError {
    pub status: hyper::StatusCode,
    pub error: Option<AwsResponseError>,
}

#[derive(Debug, Error)]
pub enum Internal {
    #[error("Internal Error: {0:?}")]
    Message(String),

    #[error("Error sending request")]
    RequestError(#[from] hyper::Error),

    #[error("Invalid header value provided: {0:?}")]
    InvalidHeaderValue(#[from] InvalidHeaderValue),

    #[error("Failed to deserialize response error")]
    DeserializeError(#[from] quick_xml::DeError),

    #[error("x-amz-date was not set when signing a request")]
    DateHeaderUnsetWhenSigning,

    #[error("Host was not set in the url")]
    HostStrUnset,

    #[error("Failed to encode x-amz-date header")]
    DateHeaderToStrError(#[from] hyper::header::ToStrError),

    #[error("Failed to parse chrono datetime in header")]
    ChronoParseError(#[from] chrono::ParseError),

    #[error("Failed to parse a number in header")]
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
}

#[derive(Debug, Error)]
pub enum Credentials {
    #[cfg(feature = "credential_file")]
    #[error("Failed to parse aws credentials file")]
    AwsCredentialsParseError,

    #[error("Could not find credentials from environment variables")]
    CouldNotFindCredentials,

    #[cfg(feature = "credential_file")]
    #[error("Found credentials file, but could not find 'aws_access_key_id' *and* 'aws_secret_access_key'")]
    AwsCredentialsNotFound,
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("Received an error while executing a request: {0:?}")]
    ResponseError(#[from] ResponseError),

    #[error("Invalid uri parts")]
    InvalidUrl(#[from] url::ParseError),

    #[error("Unsuccessful status code received. {0:?}")]
    Credentials(#[from] Credentials),

    #[error("Internal Error")]
    Internal(#[from] Internal),

    #[error("Failed to build client because not all fields were provided")]
    ClientBuildError,
}
