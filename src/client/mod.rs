use crate::{
    get_object::GetObjectResponse,
    list_buckets::Bucket,
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
use http::Uri;
use hyper::Body as HttpBody;
use std::convert::TryFrom;

mod builder;

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
    /// Create a new Client with the given parameters
    /// **NOTE:** The secret key is not stored in memory after this call.
    pub fn new<T: AsRef<str>>(
        access_key: T,
        secret_key: T,
        region: Region,
        host: T,
    ) -> Result<Self, Error> {
        let date = Utc::now();
        Ok(Self {
            client: hyper::Client::builder().build_http(),
            signing_key: SigningKey::from_date(&secret_key.as_ref(), &date.clone(), region),
            region,
            date,
            host: Uri::try_from(host.as_ref())?,
            access_key: access_key.as_ref().to_owned(),
        })
    }

    /// Helper method to construct a new builder
    pub fn builder<T: AsRef<str>>() -> Builder<T> {
        Builder::new()
    }

    /// A convience method for a `GetObject` request.
    ///
    /// Note: If more control is needed over the request parameters use the
    /// `Client::send()` method directly
    pub async fn get<T: AsRef<str>>(&self, bucket: T, key: T) -> Result<GetObjectResponse, Error> {
        let request = GetObject::new(bucket, key);
        self.send(request).await
    }

    /// A convience method for a `PutObject` request.
    ///
    /// Note: If more control is needed over the request parameters use the
    /// `Client::send()` method directly
    pub async fn put<T: AsRef<str>>(
        &self,
        bucket: T,
        key: T,
        contents: Vec<u8>,
    ) -> Result<String, Error> {
        let request = PutObject::new(bucket, key, contents);
        self.send(request).await
    }

    /// A convience method for a `DeleteObject` request.
    ///
    /// Note: If more control is needed over the request parameters use the
    /// `Client::send()` method directly
    pub async fn delete<T: AsRef<str>>(&self, bucket: T, key: T) -> Result<bool, Error> {
        let request = DeleteObject::new(bucket, key);
        self.send(request).await
    }

    /// A convience method for a `CreateBucket` request.
    ///
    /// Note: If more control is needed over the request parameters use the
    /// `Client::send()` method directly
    pub async fn create<T: AsRef<str>>(&self, bucket: T) -> Result<(), Error> {
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

        let response = self.client.request(request).await?;

        println!("{:#?}", response);

        T::into_response(response).await
    }
}
