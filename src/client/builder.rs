use super::Client;
use crate::{
    Error,
    Region,
};

#[derive(Debug)]
pub struct Builder<T: AsRef<str>> {
    region: Region,
    host: Option<T>,
    access_key: Option<T>,
    secret_key: Option<T>,
}

impl<T: AsRef<str>> Default for Builder<T> {
    fn default() -> Self {
        Self {
            region: Region::UsEast1,
            host: None,
            access_key: None,
            secret_key: None,
        }
    }
}

impl<T: AsRef<str>> Builder<T> {
    /// Construct a new Client builder
    pub fn new() -> Self {
        Builder::default()
    }

    /// Set the client region
    pub fn region(self, region: Region) -> Self {
        Self { region, ..self }
    }

    /// Set the aws s3 server uri
    pub fn host(self, host: T) -> Self {
        Self {
            host: Some(host),
            ..self
        }
    }

    /// Set the access key to be used on every request
    pub fn access_key(self, access_key: T) -> Self {
        Self {
            access_key: Some(access_key),
            ..self
        }
    }

    /// Set the access key paired secret key to sign every request.
    ///
    /// **NOTE:** The secret key is not stored in memory after
    /// a client is built.
    pub fn secret_key(self, secret_key: T) -> Self {
        Self {
            secret_key: Some(secret_key),
            ..self
        }
    }

    /// Build a client with the give settings
    pub fn build(self) -> Result<Client, Error> {
        if let (Some(access_key), Some(secret_key), Some(host)) =
            (self.access_key, self.secret_key, self.host)
        {
            Client::new(access_key, secret_key, self.region, host)
        } else {
            Err(Error::ClientBuildError)
        }
    }
}
