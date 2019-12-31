use crate::{
    get_object::{
        GetObject,
        GetObjectResponse,
    },
    put_object::PutObject,
    AwsRequest,
    Error,
    Region,
    SigningKey,
};
use chrono::{
    DateTime,
    Utc,
};
use http::Uri;
use hyper::Body as HttpBody;
use std::convert::TryFrom;

#[derive(Debug)]
pub struct ClientBuilder<T: AsRef<str>> {
    region: Region,
    host: Option<T>,
    access_key: Option<T>,
    secret_key: Option<T>,
}

impl<T: AsRef<str>> Default for ClientBuilder<T> {
    fn default() -> Self {
        Self {
            region: Region::UsEast1,
            host: None,
            access_key: None,
            secret_key: None,
        }
    }
}

impl<T: AsRef<str>> ClientBuilder<T> {
    pub fn new() -> Self {
        ClientBuilder::default()
    }

    pub fn region(self, region: Region) -> Self {
        Self { region, ..self }
    }

    pub fn host(self, host: T) -> Self {
        Self {
            host: Some(host),
            ..self
        }
    }

    pub fn access_key(self, access_key: T) -> Self {
        Self {
            access_key: Some(access_key),
            ..self
        }
    }

    pub fn secret_key(self, secret_key: T) -> Self {
        Self {
            secret_key: Some(secret_key),
            ..self
        }
    }

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

#[derive(Debug)]
pub struct Client {
    client: hyper::Client<hyper::client::HttpConnector, HttpBody>,
    access_key: String,
    signing_key: SigningKey,
    region: Region,
    date: DateTime<Utc>,
    host: Uri,
}

impl Client {
    pub fn new<T: AsRef<str>>(
        access_key: T,
        secret_key: T,
        region: Region,
        host: T,
    ) -> Result<Self, Error> {
        let date = Utc::now();
        Ok(Self {
            client: hyper::Client::builder().build_http(),
            signing_key: SigningKey::from_date(&secret_key.as_ref(), &date.clone(), region.clone()),
            region,
            date,
            host: Uri::try_from(host.as_ref())?,
            access_key: access_key.as_ref().to_owned(),
        })
    }

    pub fn builder<T: AsRef<str>>() -> ClientBuilder<T> {
        ClientBuilder::new()
    }

    pub async fn get_object<T: AsRef<str>>(
        &self,
        bucket: T,
        key: T,
    ) -> Result<GetObjectResponse, Error> {
        let request = GetObject::new(bucket, key);
        self.send(request).await
    }

    pub async fn put_object<T: AsRef<str>>(
        &self,
        bucket: T,
        key: T,
        contents: Vec<u8>,
    ) -> Result<String, Error> {
        let request = PutObject::new(bucket, key, contents);
        self.send(request).await
    }

    pub async fn send<T: AwsRequest>(&self, request: T) -> Result<T::Response, Error> {
        let request = request.into_request(
            self.host.clone(),
            &self.access_key,
            &self.signing_key,
            self.region.clone(),
        )?;

        let response = self.client.request(request).await?;

        println!("{:#?}", response);

        T::into_response(response).await
    }
}
