use crate::{region::Region, signing_key::SigningKey};
use chrono::{DateTime, Utc};

#[derive(Debug)]
pub struct Client {
    pub key: SigningKey,
    pub region: Region,
    pub date: DateTime<Utc>,
    pub host: String,
}

impl Client {
    pub fn new(secret: &str, host: String, date: DateTime<Utc>, region: Region) -> Self {
        Self {
            key: SigningKey::from_date(&secret, &date.clone(), region.clone()),
            region,
            date,
            host,
        }
    }

    // pub fn request(&self, method: Method, bucket: &str) -> CanonicalRequest {
    //     let date = format!("{}", self.date.format("%Y%m%dT%H%M%SZ"));
    //     CanonicalRequest::new(method)
    //         .add_header("x-amz-date".to_owned(), date)
    //         .add_header("host".to_owned(), format!("{}.{}", bucket, self.host))
    // }
}
