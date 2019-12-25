use crate::region::Region;
use chrono::{DateTime, Utc};
use sha2::{Digest, Sha256};

#[derive(Debug)]
pub struct StringToSign {
    pub timestamp: String,
    pub scope: String,
    pub request: String,
}

impl StringToSign {
    pub fn new(date: &DateTime<Utc>, region: Region, request: String) -> Self {
        let mut hasher = Sha256::new();
        hasher.input(request);

        let timestamp = format!("{}", date.format("%Y%m%dT%H%M%SZ"));
        let date = format!("{}", date.format("%Y%m%d"));
        let region: String = region.into();
        let scope = format!("{}/{}/s3/aws4_request", date, region);

        StringToSign {
            timestamp,
            scope,
            request: hex::encode(hasher.result().as_slice()),
        }
    }
}

impl Into<String> for StringToSign {
    fn into(self) -> String {
        format!(
            "{algorithm}\n{timestamp}\n{scope}\n{request}",
            algorithm = "AWS4-HMAC-SHA256",
            timestamp = self.timestamp,
            scope = self.scope,
            request = self.request
        )
    }
}
