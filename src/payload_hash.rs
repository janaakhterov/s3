use crate::{
    Error,
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
    fn payload_hash(self, bytes: Option<&[u8]>) -> Result<Self, Error>
    where
        Self: Sized;
}

// Hash of empty body
const NO_PAYLOAD_HASH: &str = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";

impl PayloadHash for Builder {
    fn payload_hash(self, bytes: Option<&[u8]>) -> Result<Self, Error> {
        let payload_hash = if let Some(bytes) = bytes {
            let mut hasher = Sha256::new();
            hasher.input(&bytes);
            let payload_hash = hex::encode(hasher.result().as_slice());

            HeaderValue::from_str(&payload_hash)?
        } else {
            HeaderValue::from_str(&NO_PAYLOAD_HASH)?
        };

        Ok(self.header(Headers::X_AMZ_CONTENT_SHA256, payload_hash))
    }
}
