use crate::region::Region;
use chrono::{DateTime, Utc};
use ring::hmac::{self, Key};

#[derive(Debug)]
pub struct SigningKey {
    pub key: Key,
    // date: DateTime<Utc>
}

impl SigningKey {
    pub fn from_date(secret: &str, date: &DateTime<Utc>, region: Region) -> Self {
        let key = SigningKey::generate_key(secret, &date, region);
        Self {
            key,
            // date,
        }
    }

    /// Generates a new key from a secret, date, and a region
    fn generate_key(secret: &str, date: &DateTime<Utc>, region: Region) -> Key {
        let region: String = region.into();
        let date = format!("{}", date.date().format("%Y%m%d"));

        let tag = hmac::sign(
            &Key::new(hmac::HMAC_SHA256, &format!("AWS4{}", secret).as_bytes()),
            date.as_bytes(),
        );
        let tag = hmac::sign(
            &Key::new(hmac::HMAC_SHA256, tag.as_ref()),
            region.as_bytes(),
        );
        let tag = hmac::sign(&Key::new(hmac::HMAC_SHA256, tag.as_ref()), "s3".as_bytes());
        let tag = hmac::sign(
            &Key::new(hmac::HMAC_SHA256, tag.as_ref()),
            "aws4_request".as_bytes(),
        );
        Key::new(hmac::HMAC_SHA256, tag.as_ref())
    }

    /// Signs the string and returns the hex encoded signature
    #[inline(always)]
    pub fn sign<T: AsRef<str>>(&self, s: T) -> String {
        let tag = hmac::sign(&self.key, s.as_ref().as_bytes());
        hex::encode(tag.as_ref())
    }
}
