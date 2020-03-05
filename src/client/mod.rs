use crate::{
    error,
    request::get::object::GetObjectResponse,
    types::Bucket,
    AwsRequest,
    CreateBucket,
    DeleteObject,
    Error,
    GetObject,
    ListBuckets,
    PutObject,
    Region,
    SigningKey,
};
use builder::Builder;
use chrono::{
    DateTime,
    Utc,
};
use hyper::Body as HttpBody;
use std::str::FromStr;
use url::Url;

mod builder;

static AWS_ACCESS_KEY: &str = "AWS_ACCESS_KEY";
static AWS_SECRET_KEY: &str = "AWS_SECRET_KEY";

#[cfg(feature = "credential_file")]
static AWS_SHARED_CREDENTIALS_FILE_ENV: &str = "AWS_SHARED_CREDENTIALS_FILE";
#[cfg(feature = "credential_file")]
static AWS_SHARED_CREDENTIALS_FILE: &str = "~/.aws/credentials";

#[derive(Debug)]
pub struct Client {
    client: hyper::Client<hyper::client::HttpConnector, HttpBody>,
    access_key: String,
    signing_key: SigningKey,
    region: Region,
    date: DateTime<Utc>,
    host: Url,
}

impl Client {
    /// Create a new Client with the given parameters
    /// **NOTE:** The secret key is not stored in memory after this call.
    pub fn new<T1: AsRef<str>, T2: AsRef<str>>(
        access_key: T1,
        secret_key: T1,
        region: Region,
        host: T2,
    ) -> Result<Self, Error> {
        let date = Utc::now();
        Ok(Self {
            client: hyper::Client::builder().build_http(),
            signing_key: SigningKey::from_date(&secret_key.as_ref(), &date.clone(), region),
            region,
            date,
            host: Url::from_str(host.as_ref())?,
            access_key: access_key.as_ref().to_owned(),
        })
    }

    /// A helper method for constructing a Client from either environment variables
    /// or from a file. The environment variable `AWS_SHARED_CREDENTIALS_FILE` is used
    /// to determine the file, otherwise `~/.aws/credentials` is used.
    pub fn load<T: AsRef<str>>(host: T) -> Result<Self, Error> {
        let access_key = std::env::var(AWS_ACCESS_KEY);
        let secret_key = std::env::var(AWS_SECRET_KEY);

        #[allow(unused_mut, unused_assignments)]
        let mut region = Region::UsEast1;

        #[cfg(feature = "credential_file")]
        {
            println!("Getting config file");
            region = if let Ok(contents) = std::fs::read_to_string(&std::path::Path::new(&shellexpand::tilde("~/.aws/config").to_string())) {
                println!("{:?}", contents);
                if let Some(config) = crate::parser::config::config("default", &contents)? {
                    println!("{:?}", config);
                    config.get("region").map(|&region| Region::from(region))
                } else {
                    None
                }
            } else {
                println!("Got error reading contents");
                None
            }.unwrap_or(Region::UsEast1)
        }

        if let (Ok(access_key), Ok(secret_key)) = (access_key, secret_key) {
            return Client::new(access_key, secret_key, region, host);
        }


        #[cfg(feature = "credential_file")]
        {
            let file_name = if let Ok(file_name) = std::env::var(AWS_SHARED_CREDENTIALS_FILE_ENV) {
                // This is used incase the environment variable uses '~' for home directory
                shellexpand::tilde(&file_name).to_string()
            } else {
                shellexpand::tilde(&AWS_SHARED_CREDENTIALS_FILE).to_string()
            };

            let path = std::path::Path::new(&file_name);
            if let Ok(contents) = std::fs::read_to_string(&path) {
                match crate::parser::credentials::credentials("default", &contents) {
                    // If file with a default profile and both keys were found we return a client
                    Ok(Some(cred)) => {
                        return Client::new(
                            cred.aws_access_key_id,
                            cred.aws_secret_access_key,
                            region,
                            host,
                        )
                    }

                    // If file was found and it was parsed, but no keys were found we do nothing
                    Ok(None) => {}

                    // If file was found, but was unparsable then we error
                    Err(err) => return Err(err),
                }
            }
        }

        // If we've exhausted all possible credential providers we will error out
        Err(error::Error::from(
            error::Credentials::CouldNotFindCredentials,
        ))?
    }

    /// Helper method to construct a new builder
    pub fn builder<T: AsRef<str>>() -> Builder<T> {
        Builder::new()
    }

    /// A convience method for a `GetObject` request.
    ///
    /// Note: If more control is needed over the request parameters use the
    /// `Client::send()` method directly
    pub async fn get<'a>(&self, bucket: &'a str, key: &'a str) -> Result<GetObjectResponse, Error> {
        let request = GetObject::new(bucket, key);
        self.send(request).await
    }

    /// A convience method for a `PutObject` request.
    ///
    /// Note: If more control is needed over the request parameters use the
    /// `Client::send()` method directly
    pub async fn put<'a>(
        &self,
        bucket: &'a str,
        key: &'a str,
        contents: Vec<u8>,
    ) -> Result<String, Error> {
        let request = PutObject::new(bucket, key, contents);
        self.send(request).await
    }

    /// A convience method for a `DeleteObject` request.
    ///
    /// Note: If more control is needed over the request parameters use the
    /// `Client::send()` method directly
    pub async fn delete<'a>(&self, bucket: &'a str, key: &'a str) -> Result<bool, Error> {
        let request = DeleteObject::new(bucket, key);
        self.send(request).await
    }

    /// A convience method for a `CreateBucket` request.
    ///
    /// Note: If more control is needed over the request parameters use the
    /// `Client::send()` method directly
    pub async fn create<'a>(&self, bucket: &'a str) -> Result<(), Error> {
        let request = CreateBucket::new(bucket);
        self.send(request).await
    }

    /// A convience method for a `ListBuckets` request.
    ///
    /// Note: If more control is needed over the request parameters use the
    /// `Client::send()` method directly
    pub async fn list_buckets(&self) -> Result<Vec<Bucket>, Error> {
        self.send(ListBuckets).await
    }

    /// Sends any S3 request and returns the requests response type.
    pub async fn send<T: AwsRequest>(&self, request: T) -> Result<T::Response, Error> {
        let request = request.into_request(
            self.host.clone(),
            &self.access_key,
            &self.signing_key,
            self.region.clone(),
        )?;

        println!("{:#?}", request);

        let response = self
            .client
            .request(request)
            .await
            .map_err(error::Internal::from)?;

        println!("{:#?}", response);

        T::into_response(response).await
    }
}
