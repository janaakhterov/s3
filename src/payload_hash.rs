use crate::{
    error,
    Headers,
};
use http::{
    request::Builder,
    HeaderValue,
};
use sha2::{
    Digest,
    Sha256,
};

pub trait PayloadHash {
    fn payload_hash(self, bytes: Option<&[u8]>) -> Result<Self, error::Error>
    where
        Self: Sized;
}

// Hash of empty body
const NO_PAYLOAD_HASH: &str = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";

impl PayloadHash for Builder {
    fn payload_hash(mut self, bytes: Option<&[u8]>) -> Result<Self, error::Error> {
        let payload_hash = if let Some(bytes) = bytes {
            let mut hasher = Sha256::new();
            hasher.input(&bytes);
            let payload_hash = hex::encode(hasher.result().as_slice());

            let content_md5 = base64::encode(&*md5::compute(&bytes));
            self = self.header(
                Headers::CONTENT_MD5,
                HeaderValue::from_str(&content_md5).map_err(error::Internal::from)?,
            );

            HeaderValue::from_str(&payload_hash).map_err(error::Internal::from)?
        } else {
            HeaderValue::from_str(&NO_PAYLOAD_HASH).map_err(error::Internal::from)?
        };

        Ok(self.header(Headers::X_AMZ_CONTENT_SHA256, payload_hash))
    }
}
